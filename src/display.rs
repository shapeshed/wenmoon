pub fn colourise_grey(text: &str) -> String {
    format!("\x1b[90m{}\x1b[0m", text)
}

pub fn colourise_red(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

pub fn colourise_green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

pub fn format_and_color_value(percent_change: f64, add_percentage_sign: bool) -> String {
    let formatted_string = if add_percentage_sign {
        format!("{:.2}%", percent_change)
    } else {
        format!("{:.2}", percent_change)
    };

    if percent_change > 0.0 {
        colourise_green(&formatted_string.to_string())
    } else if percent_change < 0.0 {
        colourise_red(&formatted_string.to_string())
    } else {
        colourise_grey(&formatted_string.to_string())
    }
}

pub fn format_with_commas(num: f64) -> String {
    let mut num_str = format!("{:.2}", num);

    if let Some(dot_index) = num_str.find('.') {
        let int_part = &num_str[..dot_index];
        let dec_part = &num_str[dot_index..];
        let int_part_with_commas = int_part
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",");
        num_str = format!("{}{}", int_part_with_commas, dec_part);
    }
    num_str
}

pub fn print_table_header() {
    println!(
        " +{:-<12}+{:-<10}+{:-<12}+{:-<17}+{:-<17}+{:-<17}+",
        "", "", "", "", "", ""
    );
    println!(
        " | {:<10} | {:>8} | {:>10} | {:>15} | {:>15} | {:>15} |",
        "Ticker", "Price", "+/- 24h", "Value (USD)", "P&L (USD)", "P&L %"
    );
    println!(
        " +{:-<12}+{:-<10}+{:-<12}+{:-<17}+{:-<17}+{:-<17}+",
        "", "", "", "", "", ""
    );
}

pub fn print_table_row(row: &super::TableRow) {
    let percent_change_str = format_and_color_value(row.percent_change, true);
    let pl_str = row.pl.map_or_else(
        || colourise_grey("-"),
        |pl| format_and_color_value(pl, false),
    );
    let pl_percent_str = row.pl_percent.map_or_else(
        || colourise_grey("-"),
        |pl_percent| format_and_color_value(pl_percent, true),
    );

    let value_str = row
        .value
        .map_or_else(|| "-".to_string(), format_with_commas);

    println!(
        " | {:<10} | {:>8.2} | {:>19} | {:>15} | {:>24} | {:>24} |",
        row.ticker, row.price, percent_change_str, value_str, pl_str, pl_percent_str
    );
}

pub fn print_sum_row(
    sum: f64,
    average_percent_change: f64,
    cumulative_pl: f64,
    cumulative_pl_percentage: f64,
) {
    let percent_change_str = format_and_color_value(average_percent_change, true);
    let cumulative_pl_str = format_and_color_value(cumulative_pl, false);
    let cumulative_pl_percentage_str = format_and_color_value(cumulative_pl_percentage, true);
    println!(
        " | {:-<11}+{:-<10}+{:-<12}+{:-<17}+{:-<17}+{:-<17}+",
        "", "", "", "", "", ""
    );

    println!(
        " | {:<10} | {:>8} | {:>19} | {:>15} | {:>24} | {:>24} |",
        "Total",
        "",
        percent_change_str,
        format_with_commas(sum),
        cumulative_pl_str,
        cumulative_pl_percentage_str
    );

    println!(
        " +{:-<12}+{:-<10}+{:-<12}+{:-<17}+{:-<17}+{:-<17}+",
        "", "", "", "", "", ""
    );
}
