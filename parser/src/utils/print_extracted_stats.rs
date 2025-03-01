use crate::extract::ExtractedData;
use prettytable::{
    format::{FormatBuilder, LinePosition, TableFormat},
    row, Table,
};
use std::path::Path;

pub fn print_extracted_stats(extracted: ExtractedData, output_file_path: &Path) {
    let mut table = Table::new();

    // Define a custom table format with periods for internal spacing
    let table_format: TableFormat = FormatBuilder::new()
        .column_separator('│')
        .padding(2, 2)
        .inter_row(LinePosition::Intern)
        .separators(&[LinePosition::Top, LinePosition::Bottom], "─")
        .borders('─')
        .padding_left(2)
        .padding_right(2)
        .indent(2)
        .dotted_lines(true)
        .build();
    table.set_format(table_format);

    println!("\nExtracted Data Statistics:");

    // Add a descriptive header row
    table.add_row(row!["Category", "Number of Items"]);

    // Add rows with data
    table.add_row(row!["Structs", extracted.structs.len().to_string()]);
    table.add_row(row!["Functions", extracted.functions.len().to_string()]);
    table.add_row(row!["Type Aliases", extracted.type_aliases.len().to_string()]);
    table.add_row(row!["Impls", extracted.impls.len().to_string()]);
    table.add_row(row!["Use Dependencies", extracted.use_dependencies.len().to_string()]);

    println!("Extracted data saved to {}\n", output_file_path.display());
    table.printstd();
}
