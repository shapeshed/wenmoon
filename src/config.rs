use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortfolioEntry {
    pub ticker: String,
    pub amount: Option<f64>,
    pub entry_price: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub portfolio: Vec<PortfolioEntry>,
    pub coingecko: Option<CoinGecko>,
    pub coinmarketcap: Option<CoinMarketCap>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinGecko {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinMarketCap {
    pub api_key: String,
}

impl Config {
    pub fn load(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(config_path)?;
        let mut config: Self = toml::from_str(&config_str)?;
        config.validate()?;
        for entry in &mut config.portfolio {
            entry.ticker = entry.ticker.trim().to_string();
        }
        Ok(config)
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.coingecko.is_some() && self.coinmarketcap.is_some() {
            return Err("configure only one data provider ([coingecko] or [coinmarketcap])".into());
        }

        if self.portfolio.is_empty() {
            return Err("portfolio must contain at least one entry".into());
        }

        let mut seen = HashSet::new();
        for entry in &self.portfolio {
            let ticker = entry.ticker.trim();
            if ticker.is_empty() {
                return Err("portfolio entries must have a non-empty ticker".into());
            }

            if !seen.insert(ticker.to_lowercase()) {
                return Err(format!("duplicate portfolio ticker: {ticker}").into());
            }

            if let Some(amount) = entry.amount {
                if amount < 0.0 {
                    return Err(format!("amount for {ticker} must not be negative").into());
                }
            }

            if let Some(entry_price) = entry.entry_price {
                if entry_price < 0.0 {
                    return Err(format!("entry_price for {ticker} must not be negative").into());
                }
                if entry.amount.is_none() {
                    return Err(format!("{ticker} has entry_price but no amount").into());
                }
            }
        }

        Ok(())
    }
}

pub fn get_config_path(config_arg: Option<&PathBuf>) -> PathBuf {
    if let Some(config_path) = config_arg {
        return config_path.clone();
    }

    let config_dir = env::var_os("XDG_CONFIG_HOME").map(PathBuf::from);
    #[cfg(windows)]
    let config_dir = config_dir.or_else(|| env::var_os("APPDATA").map(PathBuf::from));
    let config_dir = config_dir
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));

    config_dir.join("wenmoon").join("config.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(config: &str) -> Config {
        toml::from_str(config).expect("config should parse")
    }

    #[test]
    fn accepts_a_valid_config() {
        let config = parse(
            r#"
            [coinmarketcap]
            api_key = "key"

            [[portfolio]]
            ticker = "BTC"
            amount = 1.5
            entry_price = 20000.0
            "#,
        );

        config.validate().unwrap();
        assert_eq!(config.portfolio.len(), 1);
        assert_eq!(config.portfolio[0].ticker, "BTC");
    }

    #[test]
    fn rejects_unknown_fields() {
        let result = toml::from_str::<Config>(
            r#"
            [[portfolio]]
            ticker = "BTC"
            ticker_typo = "BTC"
            "#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_tickers() {
        let config = parse(
            r#"
            [[portfolio]]
            ticker = "btc"

            [[portfolio]]
            ticker = "BTC"
            "#,
        );

        let err = config.validate().unwrap_err().to_string();
        assert!(err.contains("duplicate portfolio ticker"));
    }

    #[test]
    fn rejects_multiple_providers() {
        let config = parse(
            r#"
            [coingecko]
            api_key = "a"

            [coinmarketcap]
            api_key = "b"

            [[portfolio]]
            ticker = "BTC"
            "#,
        );

        let err = config.validate().unwrap_err().to_string();
        assert!(err.contains("configure only one data provider"));
    }

    #[test]
    fn rejects_empty_portfolio() {
        let config = parse(
            r#"
            portfolio = []

            [coinmarketcap]
            api_key = "key"
            "#,
        );

        let err = config.validate().unwrap_err().to_string();
        assert!(err.contains("portfolio must contain at least one entry"));
    }

    #[test]
    fn rejects_entry_price_without_amount() {
        let config = parse(
            r#"
            [[portfolio]]
            ticker = "BTC"
            entry_price = 20000.0
            "#,
        );

        let err = config.validate().unwrap_err().to_string();
        assert!(err.contains("has entry_price but no amount"));
    }

    #[test]
    fn rejects_negative_amounts() {
        let config = parse(
            r#"
            [[portfolio]]
            ticker = "BTC"
            amount = -1.0
            "#,
        );

        let err = config.validate().unwrap_err().to_string();
        assert!(err.contains("must not be negative"));
    }

    #[test]
    fn explicit_config_path_wins() {
        let path = PathBuf::from("/tmp/custom/config.toml");
        assert_eq!(get_config_path(Some(&path)), path);
    }
}
