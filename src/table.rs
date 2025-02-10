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
