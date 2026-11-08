use crate::config::PortfolioEntry;
use crate::model::{sort_table_rows, TableRow};
use crate::portfolio::calculate_pl_and_percentage;
use crate::traits::FetchAndTransform;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinsWithMarketDataResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Option<u32>,
    pub fully_diluted_valuation: Option<f64>,
    pub total_volume: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub market_cap_change_percentage_24h: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: Option<f64>,
    pub ath_change_percentage: Option<f64>,
    pub ath_date: String,
    pub atl: Option<f64>,
    pub atl_change_percentage: Option<f64>,
    pub atl_date: String,
    pub roi: Option<Roi>,
    pub last_updated: String,
    pub price_change_percentage_1h_in_currency: Option<f64>,
    pub price_change_percentage_24h_in_currency: Option<f64>,
    pub price_change_percentage_7d_in_currency: Option<f64>,
    pub price_change_percentage_30d_in_currency: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Roi {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}

const BASE_URL: &str = "https://api.coingecko.com/api/v3/coins/markets";
const CURRENCY: &str = "usd";

pub struct CoinGeckoClient {
    client: Client,
    api_key: String,
    portfolio: Vec<PortfolioEntry>,
    sort_order: String,
}

impl CoinGeckoClient {
    pub fn new(api_key: String, portfolio: Vec<PortfolioEntry>, sort_order: String) -> Self {
        CoinGeckoClient {
            client: Client::new(),
            api_key,
            portfolio,
            sort_order,
        }
    }

    pub async fn fetch_and_transform(&self) -> Result<Vec<TableRow>, Box<dyn std::error::Error>> {
        // Extract ids from portfolio
        let ids = self
            .portfolio
            .iter()
            .map(|entry| entry.ticker.clone())
            .collect::<Vec<String>>()
            .join(",");
        // Fetch the raw data
        let raw_data = self
            .fetch_data(&ids)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        // Parse and transform data
        let transformed_data = self
            .transform_data(&raw_data)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(transformed_data)
    }
}

#[async_trait]
impl FetchAndTransform for CoinGeckoClient {
    async fn fetch_data(&self, ids: &str) -> Result<String, reqwest::Error> {
        let url = format!(
            "{BASE_URL}?ids={ids}&vs_currency={CURRENCY}&price_change_percentage=1h%2C24h%2C7d%2C30d"
        );

        self.client
            .get(&url)
            .header("x-cg-demo-api-key", &self.api_key)
            // API returns 403 with default header
            .header("user-agent", "curl/7.54.1")
            .send()
            .await?
            .text()
            .await
    }

    async fn transform_data(&self, data: &str) -> Result<Vec<TableRow>, serde_json::Error> {
        let responses: Vec<CoinsWithMarketDataResponse> = serde_json::from_str(data)?;

        let mut table_rows: Vec<TableRow> = self
            .portfolio
            .iter()
            .filter_map(|entry| {
                responses
                    .iter()
                    .find(|&response| response.id == entry.ticker.to_lowercase())
                    .and_then(|response| {
                        // Coins with no market data (e.g. migrated or inactive
                        // tokens) can come back with a null current price.
                        let current_price = response.current_price?;
                        let (pl, pl_percent) = entry
                            .amount
                            .map(|amount| {
                                calculate_pl_and_percentage(
                                    entry.entry_price,
                                    current_price,
                                    amount,
                                )
                            })
                            .unwrap_or((None, None));
                        Some(TableRow {
                            price: Some(current_price),
                            entry_price: entry.entry_price,
                            amount: entry.amount,
                            ticker: response.symbol.to_uppercase(),
                            hourly_percent_change: response
                                .price_change_percentage_1h_in_currency
                                .unwrap_or(0.0),
                            daily_percent_change: response.price_change_percentage_24h_in_currency,
                            weekly_percent_change: response.price_change_percentage_7d_in_currency,
                            monthly_percent_change: response
                                .price_change_percentage_30d_in_currency,
                            value: entry.amount.map(|amount| amount * current_price),
                            pl,
                            pl_percent,
                        })
                    })
            })
            .collect();

        sort_table_rows(&mut table_rows, &self.sort_order);

        Ok(table_rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> CoinGeckoClient {
        let portfolio = vec![PortfolioEntry {
            ticker: "cream-2".to_string(),
            amount: None,
            entry_price: None,
        }];
        CoinGeckoClient::new("test".to_string(), portfolio, "d".to_string())
    }

    fn coin(current_price: &str, total_volume: &str) -> String {
        format!(
            r#"[{{
                "id": "cream-2",
                "symbol": "cream",
                "name": "Cream",
                "image": "",
                "current_price": {current_price},
                "total_volume": {total_volume},
                "price_change_percentage_1h_in_currency": null,
                "price_change_percentage_24h_in_currency": null,
                "price_change_percentage_7d_in_currency": null,
                "price_change_percentage_30d_in_currency": null,
                "ath_date": "2021-01-01T00:00:00.000Z",
                "atl_date": "2021-01-01T00:00:00.000Z",
                "last_updated": "2026-11-08T00:00:00.000Z"
            }}]"#
        )
    }

    #[tokio::test]
    async fn transform_data_accepts_fractional_volume_and_null_changes() {
        let rows = client()
            .transform_data(&coin("0.472", "131.5"))
            .await
            .unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].ticker, "CREAM");
        assert_eq!(rows[0].price, Some(0.472));
        assert_eq!(rows[0].hourly_percent_change, 0.0);
        assert_eq!(rows[0].daily_percent_change, None);
    }

    #[tokio::test]
    async fn transform_data_skips_coins_without_a_price() {
        let rows = client().transform_data(&coin("null", "0.0")).await.unwrap();

        assert!(rows.is_empty());
    }
}
