use crate::extract::ExtractedData;

pub fn print_extracted_stats(extracted: ExtractedData, output_file_path: &std::path::Path) {
    println!(
        "Extracted data saved to {} with {} structs, {} functions, {} type aliases, and {} impls",
        output_file_path.display(),
        extracted_data.structs.len(),
        extracted_data.functions.len(),
        extracted_data.type_aliases.len(),
        extracted_data.impls.len()
    );
}
