use crate::extract::ExtractedData;
use prettytable::{row, Table};
use std::path::Path;

pub fn print_extracted_stats(extracted: ExtractedData, output_file_path: &Path) {
    let mut table = Table::new();

    table.add_row(row!["Category", "Count"]);

    table.add_row(row!["Structs", extracted.structs.len()]);

    table.add_row(row!["Functions", extracted.functions.len()]);

    table.add_row(row!["Type Aliases", extracted.type_aliases.len()]);

    table.add_row(row!["Impls", extracted.impls.len()]);

    table.add_row(row!["Use Dependencies", extracted.use_dependencies.len()]);

    table.add_row(row!["Mods", extracted.mods.len()]);

    table.add_row(row!["Enums", extracted.enums.len()]);
    let mut total_variants = 0;
    for enum_info in &extracted.enums {
        total_variants += enum_info.variants.len();
    }
    table.add_row(row!["Enum Variants", total_variants]);

    println!("Extracted data saved to {}", output_file_path.display());
    table.printstd();
}
