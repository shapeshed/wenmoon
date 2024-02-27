mod api;
mod config;
mod display;
mod model;
mod portfolio;
use api::fetch_price;
use config::{get_config_path, Config};
use display::{print_sum_row, print_table_header, print_table_row};
use model::TableRow;
use portfolio::{process_portfolio_data, summarize_portfolio};
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Get the config path if any
    let config_path = get_config_path(&args);

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
            let table_rows = process_portfolio_data(&config.portfolio, &response);

            print_table_header();
            for row in &table_rows {
                print_table_row(row);
            }

            let (
                total_value,
                weighted_average_percent_change,
                cumulative_pl,
                cumulative_pl_percentage,
            ) = summarize_portfolio(&table_rows);

            print_sum_row(
                total_value,
                weighted_average_percent_change,
                cumulative_pl,
                cumulative_pl_percentage,
            );
        }
        Err(err) => eprintln!("Error fetching prices: {}", err),
    }
}

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();
//     let config_path = get_config_path(&args);

//     let config = Config::load(&config_path).unwrap_or_else(|err| {
//         eprintln!("Error loading config: {}", err);
//         std::process::exit(1);
//     });

//     let client = Client::new();

//     let tickers: Vec<String> = config
//         .portfolio
//         .iter()
//         .map(|entry| entry.ticker.clone())
//         .collect();
//     let tickers_string = tickers.join(",");

//     match fetch_price(&client, &tickers_string, &config.api_key).await {
//         Ok(response) => {
//             let mut table_rows: Vec<TableRow> = Vec::new();

//             for entry in &config.portfolio {
//                 if let Some(currency) = response.data.get(&entry.ticker) {
//                     if let Some(quote) = currency.quote.get("USD") {
//                         let value = entry.amount * quote.price;

//                         let (pl, pl_percent) = match entry.entry_price {
//                             Some(entry_price) if entry_price > 0.0 => {
//                                 // Perform P&L calculations only if entry_price is meaningful
//                                 let pl = (quote.price - entry_price) * entry.amount;
//                                 let pl_percent =
//                                     ((quote.price - entry_price) / entry_price) * 100.0;
//                                 (Some(pl), Some(pl_percent))
//                             }
//                             _ => (None, None), // Skip P&L calculations if entry_price is not meaningful or not present
//                         };

//                         table_rows.push(TableRow {
//                             price: quote.price,
//                             entry_price: entry.entry_price,
//                             amount: entry.amount,
//                             ticker: entry.ticker.clone(),
//                             percent_change: quote.percent_change_24h,
//                             value,
//                             pl,
//                             pl_percent,
//                         });
//                     } else {
//                         eprintln!("No USD quote available for {}", entry.ticker);
//                     }
//                 } else {
//                     eprintln!("No data available for {}", entry.ticker);
//                 }
//             }

//             table_rows.sort_by(|a, b| {
//                 b.percent_change
//                     .partial_cmp(&a.percent_change)
//                     .unwrap_or(std::cmp::Ordering::Equal)
//             });
//             print_table_header();

//             for row in &table_rows {
//                 print_table_row(row);
//             }

//             let total_value: f64 = table_rows.iter().map(|row| row.value).sum();
//             let weighted_average_percent_change = if total_value > 0.0 {
//                 table_rows
//                     .iter()
//                     .map(|row| {
//                         let asset_value = row.value;
//                         let weight = asset_value / total_value;
//                         row.percent_change * weight
//                     })
//                     .sum()
//             } else {
//                 0.0
//             };

//             let cumulative_pl: f64 = table_rows
//                 .iter()
//                 .filter_map(|row| row.pl) // This filters out None values and unwraps Some values
//                 .sum();

//             let total_initial_value: f64 = table_rows
//                 .iter()
//                 .filter_map(|row| row.entry_price.map(|price| row.amount * price))
//                 .sum();

//             let total_current_value: f64 = table_rows.iter().map(|row| row.value).sum();

//             let cumulative_pl_percentage: f64 = if total_initial_value > 0.0 {
//                 ((total_current_value - total_initial_value) / total_initial_value) * 100.0
//             } else {
//                 0.0 // Avoid division by zero if total_initial_value is 0
//             };

//             print_sum_row(
//                 total_value,
//                 weighted_average_percent_change,
//                 cumulative_pl,
//                 cumulative_pl_percentage,
//             );
//         }
//         Err(err) => eprintln!("Error fetching prices: {}", err),
//     }
// }
