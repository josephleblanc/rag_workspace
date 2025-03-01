use crate::extract::ExtractedData;
use prettytable::{
    format::{Alignment, FormatBuilder, TableFormat},
    row, Table,
};
use std::path::Path;

pub fn print_extracted_stats(extracted: ExtractedData, output_file_path: &Path) {
    let mut table = Table::new();

    // Define a custom table format
    let table_format: TableFormat = FormatBuilder::new()
        .column_separator('│')
        .padding_left(2)
        .padding_right(2)
        .build();
    table.set_format(table_format);

    // Add a descriptive header row
    table.add_row(row!["Category", "Number of Items"]);

    // Add rows with data, right-aligning the numbers
    table.add_row(row!["Structs", extracted.structs.len().to_string()]);
    table.add_row(row!["Functions", extracted.functions.len().to_string()]);
    table.add_row(row!["Type Aliases", extracted.type_aliases.len().to_string()]);
    table.add_row(row!["Impls", extracted.impls.len().to_string()]);
    table.add_row(row!["Use Dependencies", extracted.use_dependencies.len().to_string()]);

    println!("Extracted data saved to {}", output_file_path.display());
    table.printstd();
}
