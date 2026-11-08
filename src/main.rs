mod coingecko;
mod coinmarketcap;
mod config;
mod http;
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

    #[arg(short, long, default_value = "d", value_parser = ["h", "d", "w", "m"])]
    sort: String,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config_path = get_config_path(cli.config.as_ref());

    let config = Config::load(&config_path)
        .map_err(|err| format!("failed to load {}: {err}", config_path.display()))?;

    let data = if let Some(cg_config) = &config.coingecko {
        CoinGeckoClient::new(
            cg_config.api_key.clone(),
            config.portfolio.clone(),
            cli.sort.clone(),
        )
        .fetch_and_transform()
        .await?
    } else if let Some(cmc_config) = &config.coinmarketcap {
        CoinMarketCapClient::new(
            cmc_config.api_key.clone(),
            config.portfolio.clone(),
            cli.sort.clone(),
        )
        .fetch_and_transform()
        .await?
    } else {
        return Err("no API key provided in the config".into());
    };

    println!("{}", display_table(data));

    Ok(())
}
