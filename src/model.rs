se colored::Colorize;
use tabled::Tabled;

use separator::FixedPlaceSeparatable;

#[derive(Debug, Tabled)]
pub struct TableRow {
    #[tabled(rename = "Ticker")]
    pub ticker: String,
    #[tabled(display_with = "display_price", rename = "Price")]
    pub price: Option<f64>,
    #[tabled(display_with = "two_decimals", rename = "1hr %")]
    pub hourly_percent_change: f64,
    #[tabled(display_with = "two_decimals", rename = "1d %")]
    pub daily_percent_change: f64,
    #[tabled(display_with = "two_decimals", rename = "1w %")]
    pub weekly_percent_change: f64,
    #[tabled(display_with = "two_decimals", rename = "1m %")]
    pub monthly_percent_change: f64,
    #[tabled(skip)]
    pub entry_price: Option<f64>,
    #[tabled(skip)]
    pub amount: Option<f64>,
    #[tabled(display_with = "option_two_decimals", rename = "Value $")]
    pub value: Option<f64>,
    #[tabled(display_with = "display_pl", rename = "P&L")]
    pub pl: Option<f64>,
    #[tabled(display_with = "option_two_decimals_coloured", rename = "P&L %")]
    pub pl_percent: Option<f64>,
}

fn option_two_decimals(o: &Option<f64>) -> String {
    match o {
        Some(s) => format!("{:.2}", s),
        None => "-".to_string().bright_black().to_string(),
    }
}

fn option_two_decimals_coloured(o: &Option<f64>) -> String {
    match o {
        Some(s) if *s > 0.0 => format!("{:.2}", s).green().to_string(),
        Some(s) if *s < 0.0 => format!("{:.2}", s).red().to_string(),
        Some(s) => format!("{:.2}", s).to_string().bright_black().to_string(),
        None => "-".to_string().bright_black().to_string(),
    }
}

fn display_pl(o: &Option<f64>) -> String {
    match o {
        Some(s) if *s > 0.0 => s.separated_string_with_fixed_place(2).green().to_string(),
        Some(s) if *s < 0.0 => s.separated_string_with_fixed_place(2).red().to_string(),
        Some(s) => s
            .separated_string_with_fixed_place(2)
            .bright_black()
            .to_string(),
        None => "-".to_string().bright_black().to_string(),
    }
}

fn display_price(p: &Option<f64>) -> String {
    match p {
        Some(p) => format!("{:.3}", p),
        None => "-".to_string().bright_black().to_string(),
    }
}

fn two_decimals(p: &f64) -> String {
    if *p > 0.0 {
        format!("{:.2}", p).green().to_string()
    } else {
        format!("{:.2}", p).red().to_string()
    }
}
