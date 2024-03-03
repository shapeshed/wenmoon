use crate::api::CryptoResponse;
use crate::config::PortfolioEntry;
use crate::model::TableRow;

fn calculate_pl_and_percentage(
    entry_price: Option<f64>,
    current_price: f64,
    amount: f64,
) -> (Option<f64>, Option<f64>) {
    entry_price
        .filter(|&price| price > 0.0)
        .map(|price| {
            let pl = (current_price - price) * amount;
            let pl_percent = ((current_price - price) / price) * 100.0;
            (Some(pl), Some(pl_percent))
        })
        .unwrap_or((None, None))
}

pub fn process_portfolio_data(
    portfolio: &[PortfolioEntry],
    response: &CryptoResponse,
    sort_order: &str,
) -> Vec<TableRow> {
    let mut table_rows: Vec<TableRow> = portfolio
        .iter()
        .filter_map(|entry| {
            response.data.get(&entry.ticker).and_then(|currency| {
                currency.quote.get("USD").map(|quote| {
                    let (pl, pl_percent) = entry
                        .amount
                        .map(|amount| {
                            calculate_pl_and_percentage(entry.entry_price, quote.price, amount)
                        })
                        .unwrap_or((None, None));

                    TableRow {
                        price: Some(quote.price),
                        entry_price: entry.entry_price,
                        amount: entry.amount,
                        ticker: entry.ticker.clone(),
                        hourly_percent_change: quote.percent_change_1h,
                        daily_percent_change: quote.percent_change_24h,
                        weekly_percent_change: quote.percent_change_7d,
                        monthly_percent_change: quote.percent_change_30d,
                        value: entry.amount.map(|amount| amount * quote.price),
                        pl,
                        pl_percent,
                    }
                })
            })
        })
        .collect::<Vec<_>>();

    let sort_fn = match sort_order {
        "h" => |a: &TableRow, b: &TableRow| {
            b.hourly_percent_change
                .partial_cmp(&a.hourly_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        },
        "d" => |a: &TableRow, b: &TableRow| {
            b.daily_percent_change
                .partial_cmp(&a.daily_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        },
        "w" => |a: &TableRow, b: &TableRow| {
            b.weekly_percent_change
                .partial_cmp(&a.weekly_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        },
        "m" => |a: &TableRow, b: &TableRow| {
            b.monthly_percent_change
                .partial_cmp(&a.monthly_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        },
        _ => |a: &TableRow, b: &TableRow| {
            b.daily_percent_change
                .partial_cmp(&a.daily_percent_change)
                .unwrap_or(std::cmp::Ordering::Equal)
        },
    };
    table_rows.sort_by(sort_fn);

    table_rows
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

pub fn create_summary_row(table_rows: &[TableRow]) -> TableRow {
    let (total_value, weighted_average_percent_change, cumulative_pl, cumulative_pl_percentage) =
        summarize_portfolio(table_rows);

    TableRow {
        ticker: "Summary".to_string(),
        price: None,
        hourly_percent_change: 0.0,
        daily_percent_change: weighted_average_percent_change,
        weekly_percent_change: 0.0,
        monthly_percent_change: 0.0,
        entry_price: None,
        amount: None,
        value: Some(total_value),
        pl: Some(cumulative_pl),
        pl_percent: Some(cumulative_pl_percentage),
    }
}
