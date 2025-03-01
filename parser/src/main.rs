// src/main.rs
mod debug;
mod extract;
mod traverse;
mod utils;

use crate::{extract::*, traverse::{traverse_and_parse_directory, InfoExtractor}};

use anyhow::Result;
use std::{any::Any, env, path::Path};

#[allow(unused_imports)]
use debug::{process_any_debug, process_box_take_ownership};
mod saver;

use saver::save_extracted_data;

fn main() -> Result<()> {
    // Count node kinds
    println!("Current directory: {:?}", env::current_dir()?);
    let root_directory = Path::new("../example_traverse_target/src");

    let directories_to_ignore = Some(vec!["examples".to_string(), "assets".to_string()]);

    // Create extractors
    let struct_extractor = StructInfoExtractor {};
    let function_extractor = FunctionInfoExtractor {};
    let type_alias_extractor = TypeAliasInfoExtractor {};
    let impl_extractor = ImplInfoExtractor {};
    let use_dependency_extractor = UseDependencyInfoExtractor {};
    let mod_extractor = ModInfoExtractor {};
    let enum_extractor = EnumInfoExtractor {};
    let macro_extractor = MacroInfoExtractor {};

    let extractors: Vec<&dyn InfoExtractor> = vec![
        &struct_extractor,
        &function_extractor,
        &type_alias_extractor,
        &impl_extractor,
        &use_dependency_extractor,
        &mod_extractor,
        &enum_extractor,
        &macro_extractor,
    ];

    // Traverse the directory and extract information
    let mut extracted_data = traverse_and_parse_directory(
        root_directory,
        directories_to_ignore,
        extractors.clone(),
    )?;

    // Ensure the 'data' directory exists
    let output_dir = env::current_dir()?.join("data");
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let output_file_path = output_dir.join("extracted_data.ron");
    println!("Output file path: {}", output_file_path.display());

    save_extracted_data(&extracted_data, &output_file_path)?;
    print_extracted_stats(extracted_data, &output_file_path);

    println!("Directory parsing complete.");

    Ok(())
}
