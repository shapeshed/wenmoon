use crate::config::PortfolioEntry;
use crate::http;
use crate::model::{sort_table_rows, TableRow};
use crate::portfolio::calculate_pl_and_percentage;
use crate::traits::FetchAndTransform;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct QuotesLatestResponse {
    data: HashMap<String, CryptoCurrency>,
    status: Status,
}

#[derive(Deserialize, Debug)]
struct CryptoCurrency {
    quote: HashMap<String, Quote>,
}

#[derive(Deserialize, Debug)]
struct Quote {
    price: Option<f64>,
    percent_change_1h: Option<f64>,
    percent_change_24h: Option<f64>,
    percent_change_7d: Option<f64>,
    percent_change_30d: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct Status {
    error_code: u32,
    error_message: Option<String>,
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
            .map(|entry| entry.ticker.trim().to_uppercase())
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
impl FetchAndTransform for CoinMarketCapClient {
    async fn fetch_data(&self, tickers: &str) -> Result<String, reqwest::Error> {
        let url = format!("{BASE_URL}?symbol={tickers}&convert={CURRENCY}");

        self.client
            .get(&url)
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .send()
            .await?
            .text()
            .await
    }

    async fn transform_data(&self, data: &str) -> Result<Vec<TableRow>, serde_json::Error> {
        let response: QuotesLatestResponse = serde_json::from_str(data)?;

        if response.status.error_code != 0 {
            let message = response
                .status
                .error_message
                .unwrap_or_else(|| "unknown error".to_string());
            return Err(serde::de::Error::custom(format!(
                "CoinMarketCap error {}: {message}",
                response.status.error_code
            )));
        }

        let mut missing = Vec::new();
        let mut table_rows: Vec<TableRow> = self
            .portfolio
            .iter()
            .filter_map(|entry| {
                let ticker = entry.ticker.trim().to_uppercase();
                let Some(quote) = response
                    .data
                    .get(&ticker)
                    .and_then(|currency| currency.quote.get(CURRENCY))
                else {
                    missing.push(ticker.clone());
                    return None;
                };

                // Coins without price data can come back with a null price;
                // skip them rather than failing the response.
                let Some(price) = quote.price else {
                    missing.push(ticker.clone());
                    return None;
                };

                let (pl, pl_percent) = entry
                    .amount
                    .map(|amount| calculate_pl_and_percentage(entry.entry_price, price, amount))
                    .unwrap_or((None, None));

                Some(TableRow {
                    price: Some(price),
                    entry_price: entry.entry_price,
                    amount: entry.amount,
                    ticker,
                    hourly_percent_change: quote.percent_change_1h.unwrap_or(0.0),
                    daily_percent_change: quote.percent_change_24h,
                    weekly_percent_change: quote.percent_change_7d,
                    monthly_percent_change: quote.percent_change_30d,
                    value: entry.amount.map(|amount| amount * price),
                    pl,
                    pl_percent,
                })
            })
            .collect();

        for ticker in missing {
            eprintln!("Warning: no CoinMarketCap market data for {ticker}");
        }

        sort_table_rows(&mut table_rows, &self.sort_order);

        Ok(table_rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> CoinMarketCapClient {
        let portfolio = vec![PortfolioEntry {
            ticker: "foo".to_string(),
            amount: None,
            entry_price: None,
        }];
        CoinMarketCapClient::new("test".to_string(), portfolio, "d".to_string())
    }

    fn response(price: &str) -> String {
        format!(
            r#"{{
                "data": {{
                    "FOO": {{
                        "quote": {{
                            "USD": {{
                                "price": {price},
                                "percent_change_1h": null,
                                "percent_change_24h": null,
                                "percent_change_7d": null,
                                "percent_change_30d": null
                            }}
                        }}
                    }}
                }},
                "status": {{
                    "error_code": 0,
                    "error_message": null
                }}
            }}"#
        )
    }

    #[tokio::test]
    async fn transform_data_accepts_null_changes() {
        let rows = client().transform_data(&response("1.25")).await.unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].ticker, "FOO");
        assert_eq!(rows[0].price, Some(1.25));
        assert_eq!(rows[0].hourly_percent_change, 0.0);
        assert_eq!(rows[0].daily_percent_change, None);
    }

    #[tokio::test]
    async fn transform_data_skips_coins_without_a_price() {
        let rows = client().transform_data(&response("null")).await.unwrap();

        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn transform_data_returns_api_errors() {
        let data = r#"{
            "data": {},
            "status": {
                "error_code": 1001,
                "error_message": "API key invalid"
            }
        }"#;

        let err = client().transform_data(data).await.unwrap_err().to_string();

        assert!(err.contains("1001"));
        assert!(err.contains("API key invalid"));
    }
}
