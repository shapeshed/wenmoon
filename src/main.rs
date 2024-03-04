mod coingecko;
mod coinmarketcap;
mod config;
mod model;
mod portfolio;
mod table;
mod traits;
use clap::{App, Arg};
use coingecko::CoinGeckoClient;
use coinmarketcap::CoinMarketCapClient;
use config::{get_config_path, Config};
use std::env;
use table::display_table;

#[tokio::main]
async fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sort")
                .short('s')
                .long("sort")
                .value_name("ORDER")
                .help("Sets the sort order: h for hourly, d for daily, w for weekly, m for monthly")
                .takes_value(true),
        )
        .get_matches();

    let config_path = get_config_path(matches.value_of("config"));

    let sort_order = matches.value_of("sort").unwrap_or("d");

    let config = Config::load(&config_path).unwrap_or_else(|err| {
        eprintln!("Error loading config: {}", err);
        std::process::exit(1);
    });

    if let Some(cg_config) = &config.coingecko {
        let cg_client = CoinGeckoClient::new(
            cg_config.api_key.clone(),
            config.portfolio.clone(),
            sort_order.to_string(),
        );

        match cg_client.fetch_and_transform().await {
            Ok(data) => {
                println!("{}", display_table(data));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(cmc_config) = &config.coinmarketcap {
        let cmc_client = CoinMarketCapClient::new(
            cmc_config.api_key.clone(),
            config.portfolio.clone(),
            sort_order.to_string(),
        );
        match cmc_client.fetch_and_transform().await {
            Ok(data) => {
                println!("{}", display_table(data));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        eprintln!("No API key provided in the config.");
    }
}
