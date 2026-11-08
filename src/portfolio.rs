use crate::model::TableRow;

pub fn calculate_pl_and_percentage(
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

#[derive(Debug, Default, PartialEq)]
pub struct PortfolioSummary {
    pub total_value: f64,
    pub hourly_percent_change: Option<f64>,
    pub daily_percent_change: Option<f64>,
    pub weekly_percent_change: Option<f64>,
    pub monthly_percent_change: Option<f64>,
    pub cumulative_pl: f64,
    pub cumulative_pl_percentage: f64,
}

/// Summarizes portfolio data, calculating total value, weighted average percent
/// changes, cumulative profit and loss (P&L), and cumulative P&L percentage.
pub fn summarize_portfolio(table_rows: &[TableRow]) -> PortfolioSummary {
    let total_value: f64 = table_rows.iter().filter_map(|row| row.value).sum();

    let weighted_average = |change: fn(&TableRow) -> Option<f64>| {
        if total_value <= 0.0 {
            return None;
        }

        let mut matched = false;
        let weighted_sum: f64 = table_rows
            .iter()
            .filter_map(|row| {
                let value = row.value?;
                let change = change(row)?;
                matched = true;
                Some(change * value)
            })
            .sum();

        matched.then_some(weighted_sum / total_value)
    };

    let cumulative_pl: f64 = table_rows.iter().filter_map(|row| row.pl).sum();

    let total_initial_value: f64 = table_rows
        .iter()
        .filter_map(|row| {
            row.entry_price
                .map(|price| price * row.amount.unwrap_or(0.0))
        })
        .sum();

    let cumulative_pl_percentage = if total_initial_value > 0.0 {
        ((total_value - total_initial_value) / total_initial_value) * 100.0
    } else {
        0.0
    };

    PortfolioSummary {
        total_value,
        hourly_percent_change: weighted_average(|row| Some(row.hourly_percent_change)),
        daily_percent_change: weighted_average(|row| row.daily_percent_change),
        weekly_percent_change: weighted_average(|row| row.weekly_percent_change),
        monthly_percent_change: weighted_average(|row| row.monthly_percent_change),
        cumulative_pl,
        cumulative_pl_percentage,
    }
}

pub fn create_summary_row(table_rows: &[TableRow]) -> TableRow {
    let summary = summarize_portfolio(table_rows);

    TableRow {
        ticker: "".to_string(),
        price: None,
        hourly_percent_change: summary.hourly_percent_change.unwrap_or(0.0),
        daily_percent_change: summary.daily_percent_change,
        weekly_percent_change: summary.weekly_percent_change,
        monthly_percent_change: summary.monthly_percent_change,
        entry_price: None,
        amount: None,
        value: Some(summary.total_value),
        pl: Some(summary.cumulative_pl),
        pl_percent: Some(summary.cumulative_pl_percentage),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    fn row(
        value: Option<f64>,
        hourly: f64,
        daily: Option<f64>,
        weekly: Option<f64>,
        monthly: Option<f64>,
        entry_price: Option<f64>,
        amount: Option<f64>,
        pl: Option<f64>,
    ) -> TableRow {
        TableRow {
            ticker: "TEST".to_string(),
            price: None,
            hourly_percent_change: hourly,
            daily_percent_change: daily,
            weekly_percent_change: weekly,
            monthly_percent_change: monthly,
            entry_price,
            amount,
            value,
            pl,
            pl_percent: None,
        }
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn calculates_pl_and_percentage() {
        let (pl, pl_percent) = calculate_pl_and_percentage(Some(100.0), 150.0, 2.0);

        assert_eq!(pl, Some(100.0));
        assert_eq!(pl_percent, Some(50.0));
    }

    #[test]
    fn returns_no_pl_without_an_entry_price() {
        assert_eq!(calculate_pl_and_percentage(None, 150.0, 2.0), (None, None));
    }

    #[test]
    fn summarizes_weighted_changes() {
        let rows = vec![
            row(
                Some(100.0),
                10.0,
                Some(20.0),
                Some(30.0),
                Some(40.0),
                Some(50.0),
                Some(2.0),
                Some(100.0),
            ),
            row(
                Some(300.0),
                -10.0,
                Some(-20.0),
                Some(-30.0),
                Some(-40.0),
                Some(100.0),
                Some(3.0),
                Some(0.0),
            ),
        ];

        let summary = summarize_portfolio(&rows);

        assert_close(summary.total_value, 400.0);
        assert_close(summary.hourly_percent_change.unwrap(), -5.0);
        assert_close(summary.daily_percent_change.unwrap(), -10.0);
        assert_close(summary.weekly_percent_change.unwrap(), -15.0);
        assert_close(summary.monthly_percent_change.unwrap(), -20.0);
        assert_close(summary.cumulative_pl, 100.0);
        assert_close(summary.cumulative_pl_percentage, 0.0);
    }

    #[test]
    fn summary_handles_missing_changes() {
        let rows = vec![row(Some(100.0), 5.0, None, None, None, None, None, None)];

        let summary = summarize_portfolio(&rows);

        assert_close(summary.hourly_percent_change.unwrap(), 5.0);
        assert_eq!(summary.daily_percent_change, None);
        assert_eq!(summary.weekly_percent_change, None);
        assert_eq!(summary.monthly_percent_change, None);
    }

    #[test]
    fn summary_with_no_values_is_empty() {
        let rows = vec![row(
            None,
            5.0,
            Some(10.0),
            Some(15.0),
            Some(20.0),
            None,
            None,
            None,
        )];

        let summary = summarize_portfolio(&rows);

        assert_eq!(summary.total_value, 0.0);
        assert_eq!(summary.hourly_percent_change, None);
        assert_eq!(summary.daily_percent_change, None);
        assert_eq!(summary.weekly_percent_change, None);
        assert_eq!(summary.monthly_percent_change, None);
        assert_eq!(summary.cumulative_pl_percentage, 0.0);
    }

    #[test]
    fn creates_a_summary_row() {
        let rows = vec![row(
            Some(100.0),
            10.0,
            Some(20.0),
            Some(30.0),
            Some(40.0),
            Some(50.0),
            Some(2.0),
            Some(0.0),
        )];

        let summary = create_summary_row(&rows);

        assert_eq!(summary.ticker, "");
        assert_eq!(summary.price, None);
        assert_close(summary.hourly_percent_change, 10.0);
        assert_eq!(summary.daily_percent_change, Some(20.0));
        assert_eq!(summary.weekly_percent_change, Some(30.0));
        assert_eq!(summary.monthly_percent_change, Some(40.0));
        assert_eq!(summary.value, Some(100.0));
        assert_eq!(summary.pl, Some(0.0));
        assert_eq!(summary.pl_percent, Some(0.0));
    }
}
