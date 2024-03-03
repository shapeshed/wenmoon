mod api;
mod config;
mod model;
mod portfolio;
use api::{CoinMarketCapClient, CryptoPriceFetcher};
use clap::{App, Arg};
use config::{get_config_path, Config};
use portfolio::{create_summary_row, process_portfolio_data};
use std::env;
use tabled::settings::{
    object::{Columns, Object, Rows},
    Alignment, Border, Margin, Padding, Style,
};
use tabled::Table;

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

    // Use the get_config_path function to get the path to the config file
    let config_path = get_config_path(matches.value_of("config"));

    // Get the sort order if provided, defaulting to 'd' for daily
    let sort_order = matches.value_of("sort").unwrap_or("d");

    // Load the config file
    let config = Config::load(&config_path).unwrap_or_else(|err| {
        eprintln!("Error loading config: {}", err);
        std::process::exit(1);
    });

    // Initialise a reqwest client
    let api_client = CoinMarketCapClient::new(config.api_key.clone());

    // Collect the tickers to look up
    let tickers: Vec<String> = config
        .portfolio
        .iter()
        .map(|entry| entry.ticker.clone())
        .collect();
    let tickers_string = tickers.join(",");

    // Make the api request and handle data
    match api_client.fetch_price(&tickers_string).await {
        Ok(response) => {
            let mut table_rows = process_portfolio_data(&config.portfolio, &response, sort_order);

            let summary_row = create_summary_row(&table_rows);
            table_rows.push(summary_row);

            let table = Table::new(&table_rows)
                .with(Style::psql())
                .with(Margin::new(1, 0, 1, 0))
                .modify(
                    Columns::new(1..).not(Columns::first()),
                    Padding::new(5, 1, 0, 0),
                )
                .modify(Columns::new(1..).not(Columns::first()), Alignment::right())
                .modify(Rows::last(), Border::new().set_top('-').set_bottom('-'))
                .to_string();

            println!("{}", table);
        }
        Err(err) => eprintln!("Error fetching prices: {}", err),
    }
}
