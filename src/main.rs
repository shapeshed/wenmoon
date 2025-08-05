mod coingecko;
mod coinmarketcap;
mod config;
mod model;
mod portfolio;
mod table;
mod traits;
use clap::Parser;
use coingecko::CoinGeckoClient;
use coinmarketcap::CoinMarketCapClient;
use config::{get_config_path, Config};
use std::path::PathBuf;
use table::display_table;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, default_value = "d")]
    sort: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config_path = get_config_path(cli.config.as_ref());

    let config = Config::load(&config_path).unwrap_or_else(|err| {
        eprintln!("Error loading config: {err}");
        std::process::exit(1);
    });

    if let Some(cg_config) = &config.coingecko {
        let cg_client = CoinGeckoClient::new(
            cg_config.api_key.clone(),
            config.portfolio.clone(),
            cli.sort,
        );

        match cg_client.fetch_and_transform().await {
            Ok(data) => {
                println!("{}", display_table(data));
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    } else if let Some(cmc_config) = &config.coinmarketcap {
        let cmc_client = CoinMarketCapClient::new(
            cmc_config.api_key.clone(),
            config.portfolio.clone(),
            cli.sort,
        );
        match cmc_client.fetch_and_transform().await {
            Ok(data) => {
                println!("{}", display_table(data));
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    } else {
        eprintln!("No API key provided in the config.");
    }
}
