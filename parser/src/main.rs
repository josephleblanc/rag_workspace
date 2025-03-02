// src/main.rs
mod debug;
mod extract;
mod traverse;
mod utils;

use crate::{
    extract::*,
    traverse::{traverse_and_parse_directory, InfoExtractor},
    utils::print_extracted_stats::print_extracted_stats,
};

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
    let extracted_data =
        traverse_and_parse_directory(root_directory, directories_to_ignore, extractors.clone())?;

    // Ensure the 'data' directory exists
    let output_dir = env::current_dir()?.join("data");
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let output_file_path = output_dir.join("extracted_data.ron");
    println!("Output file path: {}", output_file_path.display());

    // Read the code from the file
    let code = std::fs::read_to_string(root_directory.join("main.rs"))?;

    println!("--- Printing one of each type ---");

    if let Some(item) = extracted_data.structs.first() {
        println!("Struct: {}", item.print_block(&code));
    } else {
        println!("No structs found.");
    }

    if let Some(item) = extracted_data.functions.first() {
        println!("Function: {}", item.print_block(&code));
    } else {
        println!("No functions found.");
    }

    if let Some(item) = extracted_data.type_aliases.first() {
        println!("Type Alias: {}", item.print_block(&code));
    } else {
        println!("No type aliases found.");
    }

    if let Some(item) = extracted_data.impls.first() {
        println!("Impl: {}", item.print_block(&code));
    } else {
        println!("No impls found.");
    }

    if let Some(item) = extracted_data.use_dependencies.first() {
        println!("Use Dependency: {}", item.print_block(&code));
    } else {
        println!("No use dependencies found.");
    }

    if let Some(item) = extracted_data.mods.first() {
        println!("Mod: {}", item.print_block(&code));
    } else {
        println!("No mods found.");
    }

    if let Some(item) = extracted_data.enums.first() {
        println!("Enum: {}", item.print_block(&code));
    } else {
        println!("No enums found.");
    }

    if let Some(item) = extracted_data.macros.first() {
        println!("Macro: {}", item.print_block(&code));
    } else {
        println!("No macros found.");
    }

    save_extracted_data(&extracted_data, &output_file_path)?;
    print_extracted_stats(extracted_data, &output_file_path);

    println!("Directory parsing complete.");

    Ok(())
}
