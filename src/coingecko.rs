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
    pub current_price: f64,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Option<u32>,
    pub fully_diluted_valuation: Option<u64>,
    pub total_volume: u64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub price_change_24h: f64,
    pub price_change_percentage_24h: f64,
    pub market_cap_change_24h: f64,
    pub market_cap_change_percentage_24h: f64,
    pub circulating_supply: f64,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: f64,
    pub ath_change_percentage: f64,
    pub ath_date: String,
    pub atl: f64,
    pub atl_change_percentage: f64,
    pub atl_date: String,
    pub roi: Option<Roi>,
    pub last_updated: String,
    pub price_change_percentage_1h_in_currency: f64,
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
                    .map(|response| {
                        let (pl, pl_percent) = entry
                            .amount
                            .map(|amount| {
                                calculate_pl_and_percentage(
                                    entry.entry_price,
                                    response.current_price,
                                    amount,
                                )
                            })
                            .unwrap_or((None, None));
                        TableRow {
                            price: Some(response.current_price),
                            entry_price: entry.entry_price,
                            amount: entry.amount,
                            ticker: response.symbol.to_uppercase(),
                            hourly_percent_change: response.price_change_percentage_1h_in_currency,
                            daily_percent_change: response.price_change_percentage_24h_in_currency,
                            weekly_percent_change: response.price_change_percentage_7d_in_currency,
                            monthly_percent_change: response
                                .price_change_percentage_30d_in_currency,
                            value: entry.amount.map(|amount| amount * response.current_price),
                            pl,
                            pl_percent,
                        }
                    })
            })
            .collect();

        sort_table_rows(&mut table_rows, &self.sort_order);

        Ok(table_rows)
    }
}
