use crate::config::PortfolioEntry;
use crate::http;
use crate::model::{sort_table_rows, TableRow};
use crate::portfolio::calculate_pl_and_percentage;
use crate::traits::FetchAndTransform;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CoinsWithMarketDataResponse {
    id: String,
    symbol: String,
    current_price: Option<f64>,
    price_change_percentage_1h_in_currency: Option<f64>,
    price_change_percentage_24h_in_currency: Option<f64>,
    price_change_percentage_7d_in_currency: Option<f64>,
    price_change_percentage_30d_in_currency: Option<f64>,
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
            client: http::client(),
            api_key,
            portfolio,
            sort_order,
        }
    }

    pub async fn fetch_and_transform(&self) -> Result<Vec<TableRow>, Box<dyn std::error::Error>> {
        let ids = self
            .portfolio
            .iter()
            .map(|entry| entry.ticker.trim().to_lowercase())
            .collect::<Vec<String>>()
            .join(",");
        let raw_data = self
            .fetch_data(&ids)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
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
            .send()
            .await?
            .text()
            .await
    }

    async fn transform_data(&self, data: &str) -> Result<Vec<TableRow>, serde_json::Error> {
        let responses: Vec<CoinsWithMarketDataResponse> = serde_json::from_str(data)?;
        let mut missing = Vec::new();

        let mut table_rows: Vec<TableRow> = self
            .portfolio
            .iter()
            .filter_map(|entry| {
                let ticker = entry.ticker.trim().to_lowercase();
                let Some(response) = responses.iter().find(|response| response.id == ticker) else {
                    missing.push(entry.ticker.clone());
                    return None;
                };

                // Coins with no market data (e.g. migrated or inactive
                // tokens) can come back with a null current price.
                let Some(current_price) = response.current_price else {
                    missing.push(entry.ticker.clone());
                    return None;
                };

                let (pl, pl_percent) = entry
                    .amount
                    .map(|amount| {
                        calculate_pl_and_percentage(entry.entry_price, current_price, amount)
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
                    monthly_percent_change: response.price_change_percentage_30d_in_currency,
                    value: entry.amount.map(|amount| amount * current_price),
                    pl,
                    pl_percent,
                })
            })
            .collect();

        for ticker in missing {
            eprintln!("Warning: no CoinGecko market data for {ticker}");
        }

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

    fn coin(current_price: &str) -> String {
        format!(
            r#"[{{
                "id": "cream-2",
                "symbol": "cream",
                "current_price": {current_price},
                "price_change_percentage_1h_in_currency": null,
                "price_change_percentage_24h_in_currency": null,
                "price_change_percentage_7d_in_currency": null,
                "price_change_percentage_30d_in_currency": null
            }}]"#
        )
    }

    #[tokio::test]
    async fn transform_data_accepts_missing_changes() {
        let rows = client().transform_data(&coin("0.472")).await.unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].ticker, "CREAM");
        assert_eq!(rows[0].price, Some(0.472));
        assert_eq!(rows[0].hourly_percent_change, 0.0);
        assert_eq!(rows[0].daily_percent_change, None);
    }

    #[tokio::test]
    async fn transform_data_skips_coins_without_a_price() {
        let rows = client().transform_data(&coin("null")).await.unwrap();

        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn transform_data_skips_unknown_tickers() {
        let client = CoinGeckoClient::new(
            "test".to_string(),
            vec![PortfolioEntry {
                ticker: "not-a-coin".to_string(),
                amount: None,
                entry_price: None,
            }],
            "d".to_string(),
        );

        let rows = client.transform_data(&coin("0.472")).await.unwrap();

        assert!(rows.is_empty());
    }
}
