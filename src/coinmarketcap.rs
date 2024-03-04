use crate::config::PortfolioEntry;
use crate::model::{sort_table_rows, TableRow};
use crate::portfolio::calculate_pl_and_percentage;
use crate::traits::FetchAndTransform;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct QuotesLatestResponse {
    pub data: HashMap<String, CryptoCurrency>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoCurrency {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub is_active: u8,
    pub is_fiat: u8,
    pub circulating_supply: f64,
    pub total_supply: f64,
    pub max_supply: Option<f64>,
    pub date_added: String,
    pub num_market_pairs: u32,
    pub cmc_rank: u32,
    pub last_updated: String,
    pub tags: Vec<String>,
    pub platform: Option<serde_json::Value>,
    pub quote: HashMap<String, Quote>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub price: f64,
    pub volume_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub percent_change_30d: f64,
    pub market_cap: f64,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub timestamp: String,
    pub error_code: u32,
    pub error_message: Option<String>,
    pub elapsed: u32,
    pub credit_count: u32,
    pub notice: Option<String>,
}

const BASE_URL: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";
const CURRENCY: &str = "USD";

pub struct CoinMarketCapClient {
    client: Client,
    api_key: String,
    portfolio: Vec<PortfolioEntry>,
    sort_order: String,
}

impl CoinMarketCapClient {
    pub fn new(api_key: String, portfolio: Vec<PortfolioEntry>, sort_order: String) -> Self {
        CoinMarketCapClient {
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
impl FetchAndTransform for CoinMarketCapClient {
    async fn fetch_data(&self, tickers: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}?symbol={}&convert={}", BASE_URL, tickers, CURRENCY);

        self.client
            .get(&url)
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .header("user-agent", "curl/7.54.1")
            .send()
            .await?
            .text()
            .await
    }

    async fn transform_data(&self, data: &str) -> Result<Vec<TableRow>, serde_json::Error> {
        let response: QuotesLatestResponse = serde_json::from_str(data)?;
        let mut table_rows: Vec<TableRow> = self
            .portfolio
            .iter()
            .filter_map(|entry| {
                response.data.get(&entry.ticker).and_then(|currency| {
                    currency.quote.get("USD").map(|quote| {
                        let (pl, pl_percent) = entry
                            .amount
                            .map(|amount| {
                                calculate_pl_and_percentage(entry.entry_price, quote.price, amount)
                            })
                            .unwrap_or((None, None));

                        TableRow {
                            price: Some(quote.price),
                            entry_price: entry.entry_price,
                            amount: entry.amount,
                            ticker: entry.ticker.clone(),
                            hourly_percent_change: quote.percent_change_1h,
                            daily_percent_change: Some(quote.percent_change_24h),
                            weekly_percent_change: Some(quote.percent_change_7d),
                            monthly_percent_change: Some(quote.percent_change_30d),
                            value: entry.amount.map(|amount| amount * quote.price),
                            pl,
                            pl_percent,
                        }
                    })
                })
            })
            .collect::<Vec<_>>();

        sort_table_rows(&mut table_rows, &self.sort_order);

        Ok(table_rows)
    }
}
