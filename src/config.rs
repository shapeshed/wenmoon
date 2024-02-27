use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioEntry {
    pub ticker: String,
    pub amount: Option<f64>,
    pub entry_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub portfolio: Vec<PortfolioEntry>,
}

impl Config {
    pub fn load(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(config_path)?;
        let config: Self = serde_yaml::from_str(&config_str)?;
        Ok(config)
    }
}

pub fn get_config_path(args: &[String]) -> String {
    let config_arg = args
        .iter()
        .position(|arg| arg == "-c")
        .map(|index| index + 1);
    if let Some(config_index) = config_arg {
        if config_index < args.len() {
            return args[config_index].clone();
        }
    }

    let xdg_config_home = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| env::var("HOME").unwrap_or_else(|_| ".".into()) + "/.config");

    format!("{}/wenmoon/config.yml", xdg_config_home)
}