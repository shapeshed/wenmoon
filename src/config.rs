use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortfolioEntry {
    pub ticker: String,
    pub amount: Option<f64>,
    pub entry_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub portfolio: Vec<PortfolioEntry>,
    pub coingecko: Option<CoinGecko>,
    pub coinmarketcap: Option<CoinMarketCap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinGecko {
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinMarketCap {
    pub api_key: String,
}

impl Config {
    pub fn load(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(config_path)?;
        let config: Self = serde_yaml::from_str(&config_str)?;
        Ok(config)
    }
}

pub fn get_config_path(config_arg: Option<&str>) -> String {
    if let Some(config_path) = config_arg {
        return config_path.to_string();
    }

    let xdg_config_home = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| env::var("HOME").unwrap_or_else(|_| ".".into()) + "/.config");

    format!("{}/wenmoon/config.yml", xdg_config_home)
}
