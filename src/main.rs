mod api;
mod config;
mod model;
mod portfolio;
use api::fetch_price;
use clap::{App, Arg};
use config::{get_config_path, Config};
use model::TableRow;
use portfolio::{process_portfolio_data, summarize_portfolio};
use reqwest::Client;
use std::env;
use tabled::settings::{
    object::{Columns, Object, Rows},
    Alignment, Border, Padding, Style,
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
                .help("Sets the sort order: d for daily, m for monthly, y for yearly")
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
    let client = Client::new();

    // Collect the tickers to look up
    let tickers: Vec<String> = config
        .portfolio
        .iter()
        .map(|entry| entry.ticker.clone())
        .collect();
    let tickers_string = tickers.join(",");

    // Make the api request and handle data
    match fetch_price(&client, &tickers_string, &config.api_key).await {
        Ok(response) => {
            let mut table_rows = process_portfolio_data(&config.portfolio, &response, sort_order);
            let (
                total_value,
                weighted_average_percent_change,
                cumulative_pl,
                cumulative_pl_percentage,
            ) = summarize_portfolio(&table_rows);
            // Create a summary TableRow
            let summary_row = TableRow {
                ticker: "Summary".to_string(),
                price: None,
                hourly_percent_change: 0.0,
                daily_percent_change: weighted_average_percent_change,
                weekly_percent_change: 0.0,
                entry_price: None,
                amount: None,
                value: Some(total_value),
                pl: Some(cumulative_pl),
                pl_percent: Some(cumulative_pl_percentage),
            };

            table_rows.push(summary_row);

            let table = Table::new(&table_rows)
                .with(Style::psql())
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
