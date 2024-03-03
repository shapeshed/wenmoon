use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoResponse {
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
    pub quote: HashMap<String, CurrencyQuote>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyQuote {
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

#[async_trait]
pub trait CryptoPriceFetcher {
    async fn fetch_price(&self, tickers: &str) -> Result<CryptoResponse, reqwest::Error>;
}

pub struct CoinMarketCapClient {
    client: Client,
    api_key: String,
}

impl CoinMarketCapClient {
    pub fn new(api_key: String) -> Self {
        CoinMarketCapClient {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl CryptoPriceFetcher for CoinMarketCapClient {
    async fn fetch_price(&self, tickers: &str) -> Result<CryptoResponse, reqwest::Error> {
        let url = format!("{}?symbol={}&convert={}", BASE_URL, tickers, CURRENCY);

        self.client
            .get(&url)
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .send()
            .await?
            .json::<CryptoResponse>()
            .await
    }
}
