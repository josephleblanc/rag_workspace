use crate::extract::ExtractedData;
use prettytable::{Cell, Row, Table};
use std::path::Path;

pub fn print_extracted_stats(extracted: ExtractedData, output_file_path: &Path) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Category"),
        Cell::new("Count"),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Structs"),
        Cell::new(&extracted.structs.len().to_string()),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Functions"),
        Cell::new(&extracted.functions.len().to_string()),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Type Aliases"),
        Cell::new(&extracted.type_aliases.len().to_string()),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Impls"),
        Cell::new(&extracted.impls.len().to_string()),
    ]));

    println!("Extracted data saved to {}", output_file_path.display());
    table.printstd();
}
