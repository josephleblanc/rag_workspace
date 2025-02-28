use crate::extract::ExtractedData;

pub fn print_extracted_stats(extracted: ExtractedData, output_file_path: &std::path::Path) {
    println!(
        "Extracted data saved to {} with {} structs, {} functions, {} type aliases, and {} impls",
        output_file_path.display(),
        extracted.structs.len(),
        extracted.functions.len(),
        extracted.type_aliases.len(),
        extracted.impls.len()
    );
}
