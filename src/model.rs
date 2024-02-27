pub struct TableRow {
    pub price: f64,
    pub entry_price: Option<f64>,
    pub amount: Option<f64>,
    pub ticker: String,
    pub percent_change: f64,
    pub value: Option<f64>,
    pub pl: Option<f64>,
    pub pl_percent: Option<f64>,
}
