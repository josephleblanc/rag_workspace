// src/main.rs
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs;

mod extract;
mod function_extractor;
mod impl_extractor;
mod struct_extractor;
mod traverse;

use anyhow::Result;
// use serde::{Deserialize, Serialize}; // Removed as they are not directly used here
// use tree_sitter::Parser; // Removed as it is not directly used here
+use std::collections::HashMap;

use extract::{ExtractedData, FunctionInfo, ImplInfo, StructInfo, TypeAliasInfo}; // Import FunctionInfo from extract
use traverse::{
    traverse_and_parse_directory, FunctionInfoExtractor, ImplInfoExtractor, InfoExtractor,
    StructInfoExtractor, TypeAliasInfoExtractor,
};

fn main() -> Result<()> {
    println!("Current directory: {:?}", env::current_dir()?);
    let root_directory = Path::new("../example_traverse_target/src");

    let directories_to_ignore = Some(vec!["examples".to_string(), "assets".to_string()]);

    // Create extractors
    let struct_extractor = StructInfoExtractor {};
    let function_extractor = FunctionInfoExtractor {};
    let type_alias_extractor = TypeAliasInfoExtractor {};
    let impl_extractor = ImplInfoExtractor {};

    let extractors: Vec<&dyn InfoExtractor> = vec![
        &struct_extractor,
        &function_extractor,
        &type_alias_extractor,
        &impl_extractor,
    ];

    // Traverse the directory and extract information
    let results = traverse_and_parse_directory(root_directory, directories_to_ignore, extractors)?;

    // Process the results
    println!("\n--- Extracted Information ---");
    let mut extracted_data = ExtractedData::default();

    for result in results {
        if let Some(struct_info) = result.downcast_ref::<StructInfo>() {
            extracted_data.structs.push(struct_info.clone());
        } else if let Some(function_info) = result.downcast_ref::<FunctionInfo>() {
            extracted_data.functions.push(function_info.clone());
        } else if let Some(type_alias_info) = result.downcast_ref::<TypeAliasInfo>() {
            extracted_data.type_aliases.push(type_alias_info.clone());
        } else if let Some(impl_info) = result.downcast_ref::<ImplInfo>() {
            extracted_data.impls.push(impl_info.clone());
        } else {
            // println!("  Unknown type of info extracted");
        }
    }
    println!("--- End Extracted Information ---");

    // Serialize to RON and save to file
    let ron_string =
        ron::ser::to_string_pretty(&extracted_data, ron::ser::PrettyConfig::default())?;

    // Ensure the 'data' directory exists
    let output_dir = env::current_dir()?.join("data");
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let output_file_path = env::current_dir()?.join("data").join("extracted_data.ron");
    println!("Output file path: {}", output_file_path.display());
    let mut file = File::create(&output_file_path)?;
    file.write_all(ron_string.as_bytes())?;

    println!(
        "Extracted data saved to {} with {} structs, {} functions, {} type aliases, and {} impls",
        output_file_path.display(),
        extracted_data.structs.len(),
        extracted_data.functions.len(),
        extracted_data.type_aliases.len(),
        extracted_data.impls.len()
    );

    println!("Directory parsing complete.");

    Ok(())
}
