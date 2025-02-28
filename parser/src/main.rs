// src/main.rs
mod debug;
mod extract;
mod traverse;
mod utils;

use crate::{
    extract::{
        ExtractedData, FunctionInfo, FunctionInfoExtractor, ImplInfo, ImplInfoExtractor,
        StructInfo, StructInfoExtractor, TypeAliasInfo, TypeAliasInfoExtractor,
    },
    print_extracted_stats::print_extracted_stats,
    traverse::{traverse_and_parse_directory, InfoExtractor},
    utils::print_extracted_stats,
};

use anyhow::Result;
use std::{any::Any, env, fs::File, io::Write, path::Path};

#[allow(unused_imports)]
use debug::{process_any_debug, process_box_take_ownership};

fn main() -> Result<()> {
    // Count node kinds
    let root_directory = Path::new("../example_traverse_target/src");
    let node_kind_counts = traverse::traverse_and_count_node_kinds(root_directory, None, vec![])?;
    println!("Node kind counts: {:?}", node_kind_counts);

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
    let results =
        traverse_and_parse_directory(root_directory, directories_to_ignore, extractors.clone())?;

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
            println!("  Unknown type of info extracted");
            let uncertain_id = (result).type_id();
            println!("  Unceratin Type: {:?}", uncertain_id);
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
    print_extracted_stats(&extracted_data, &output_file_path);

    println!("Directory parsing complete.");

    Ok(())
}
