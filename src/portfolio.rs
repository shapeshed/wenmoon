use crate::api::CryptoResponse;
use crate::config::PortfolioEntry;
use crate::model::TableRow;

pub fn process_portfolio_data(
    portfolio: &[PortfolioEntry],
    response: &CryptoResponse,
    sort_order: &str,
) -> Vec<TableRow> {
    let mut table_rows: Vec<TableRow> = Vec::new();

    for entry in portfolio {
        if let Some(currency) = response.data.get(&entry.ticker) {
            if let Some(quote) = currency.quote.get("USD") {
                // Check if amount is present and handle calculations accordingly
                if let Some(amount) = entry.amount {
                    let value = amount * quote.price;

                    let (pl, pl_percent) = entry
                        .entry_price
                        .filter(|&entry_price| entry_price > 0.0)
                        .map_or((None, None), |entry_price| {
                            let pl = (quote.price - entry_price) * amount;
                            let pl_percent = ((quote.price - entry_price) / entry_price) * 100.0;
                            (Some(pl), Some(pl_percent))
                        });

                    table_rows.push(TableRow {
                        price: Some(quote.price),
                        entry_price: entry.entry_price,
                        // Since amount is now an Option, we need to provide it directly
                        amount: Some(amount),
                        ticker: entry.ticker.clone(),
                        hourly_percent_change: quote.percent_change_1h,
                        daily_percent_change: quote.percent_change_24h,
                        weekly_percent_change: quote.percent_change_7d,
                        value: Some(value),
                        pl,
                        pl_percent,
                    });
                } else {
                    // Handle case where amount is None - possibly push a row with default or None values
                    table_rows.push(TableRow {
                        price: Some(quote.price),
                        entry_price: entry.entry_price,
                        amount: None,
                        ticker: entry.ticker.clone(),
                        hourly_percent_change: quote.percent_change_1h,
                        daily_percent_change: quote.percent_change_24h,
                        weekly_percent_change: quote.percent_change_7d,
                        value: None,
                        pl: None,
                        pl_percent: None,
                    });
                }
            }
        }
    }

    match sort_order {
        "h" => table_rows.sort_by(|a, b| {
            b.hourly_percent_change
                .partial_cmp(&a.hourly_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        "d" => table_rows.sort_by(|a, b| {
            b.daily_percent_change
                .partial_cmp(&a.daily_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        "w" => table_rows.sort_by(|a, b| {
            b.weekly_percent_change
                .partial_cmp(&a.weekly_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        _ => table_rows.sort_by(|a, b| {
            b.daily_percent_change
                .partial_cmp(&a.daily_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
    };

    table_rows

    // table_rows.sort_by(|a, b| {
    //     b.daily_percent_change
    //         .partial_cmp(&a.daily_percent_change)
    //         .unwrap_or(std::cmp::Ordering::Equal)
    // });

    // table_rows
}

/// Summarizes portfolio data, calculating total value, weighted average percent change,
/// cumulative profit and loss (P&L), and cumulative P&L percentage.
pub fn summarize_portfolio(table_rows: &[TableRow]) -> (f64, f64, f64, f64) {
    let total_value: f64 = table_rows.iter().filter_map(|row| row.value).sum();

    let weighted_average_percent_change: f64 = if total_value > 0.0 {
        table_rows
            .iter()
            .map(|row| row.hourly_percent_change * (row.value.unwrap_or(0.0) / total_value))
            .sum()
    } else {
        0.0
    };

    let cumulative_pl: f64 = table_rows.iter().filter_map(|row| row.pl).sum();

    let total_initial_value: f64 = table_rows
        .iter()
        .filter_map(|row| {
            row.entry_price
                .map(|price| price * row.amount.unwrap_or(0.0))
        })
        .sum();

    let cumulative_pl_percentage: f64 = if total_initial_value > 0.0 {
        ((total_value - total_initial_value) / total_initial_value) * 100.0
    } else {
        0.0
    };

    (
        total_value,
        weighted_average_percent_change,
        cumulative_pl,
        cumulative_pl_percentage,
    )
}
