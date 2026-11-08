use crate::model;
use crate::portfolio;
use model::TableRow;
use portfolio::create_summary_row;
use tabled::settings::{
    object::{Columns, Object, Rows},
    Alignment, Border, Margin, Padding, Style,
};
use tabled::Table;

pub fn display_table(mut data: Vec<TableRow>) -> String {
    let summary_row = create_summary_row(&data);
    data.push(summary_row);
    let table = Table::new(&data)
        .with(Style::psql())
        .with(Margin::new(1, 0, 1, 0))
        .modify(
            Columns::new(1..).not(Columns::first()),
            Padding::new(5, 1, 0, 0),
        )
        .modify(Columns::new(1..).not(Columns::first()), Alignment::right())
        .modify(Rows::last(), Border::new().top('-').bottom('-'))
        .to_string();

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    fn row(
        ticker: &str,
        price: f64,
        hourly: f64,
        daily: f64,
        weekly: f64,
        monthly: f64,
        entry_price: f64,
        amount: f64,
        value: f64,
        pl: f64,
        pl_percent: f64,
    ) -> TableRow {
        TableRow {
            ticker: ticker.to_string(),
            price: Some(price),
            hourly_percent_change: hourly,
            daily_percent_change: Some(daily),
            weekly_percent_change: Some(weekly),
            monthly_percent_change: Some(monthly),
            entry_price: Some(entry_price),
            amount: Some(amount),
            value: Some(value),
            pl: Some(pl),
            pl_percent: Some(pl_percent),
        }
    }

    #[test]
    fn renders_rows_and_a_summary_row() {
        colored::control::set_override(false);

        let output = display_table(vec![row(
            "BTC", 100.0, 1.0, 2.0, 3.0, 4.0, 50.0, 2.0, 200.0, 100.0, 100.0,
        )]);

        assert!(output.contains("Ticker"));
        assert!(output.contains("BTC"));
        assert!(output.contains("100.000"));
        assert!(output.contains("200.00"));
        assert!(output.contains("P&L"));
        assert_eq!(output.matches("BTC").count(), 1);
    }
}
