// src/main.rs
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tree_sitter::Parser;

mod extract;
mod function_extractor;
mod impl_extractor;
mod struct_extractor;
mod traverse;

use anyhow::Result;
use ron::ser::PrettyConfig;
// use serde::{Deserialize, Serialize}; // Removed as they are not directly used here
// use tree_sitter::Parser; // Removed as it is not directly used here

use extract::{ExtractedData, FunctionInfo, ImplInfo, StructInfo, TypeAliasInfo}; // Import FunctionInfo from extract
use traverse::{
    traverse_and_parse_directory, FunctionInfoExtractor, InfoExtractor,
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
            println!("  Found struct: {:?}", struct_info);
            extracted_data.structs.push(struct_info.clone());
        } else if let Some(function_info) = result.downcast_ref::<FunctionInfo>() {
            println!("  Found function: {:?}", function_info);
            extracted_data.functions.push(function_info.clone());
        } else if let Some(type_alias_info) = result.downcast_ref::<TypeAliasInfo>() {
            println!("  Found type alias: {:?}", type_alias_info);
            extracted_data.type_aliases.push(type_alias_info.clone());
        } else if let Some(impl_info) = result.downcast_ref::<ImplInfo>() {
            println!("  Found impl: {:?}", impl_info);
            extracted_data.impls.push(impl_info.clone());
        } else {
            println!("  Unknown type of info extracted");
        }
    }
    println!("--- End Extracted Information ---");

    // Serialize to RON and save to file
    let ron_string =
        ron::ser::to_string_pretty(&extracted_data, ron::ser::PrettyConfig::default())?;
    let output_file_path = env::current_dir()?.join("data").join("extracted_data.ron");
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

    // --- Single file parsing section ---
    println!("\n--- Single File Parsing ---");
    let code_snippet = r#"
        /// A [`Handle`] to the [`AnimationGraph`] to be used by the [`AnimationPlayer`](crate::AnimationPlayer) on the same entity.
        #[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, From)]
        #[reflect(Component, Default)]
        pub struct AnimationGraphHandle(pub Handle<AnimationGraph>);

        struct AnotherStruct {
            field1: i32,
        }
        type TestPoint = (u8, u8);

        fn hello_world() -> String {
            return "Hello World".to_string();
        }
    "#;

    let mut parser = Parser::new();
    if let Err(e) = parser.set_language(&tree_sitter_rust::LANGUAGE.into()) {
        eprintln!("Error loading Rust grammar: {}", e);
        return Err(e.into());
    }

    let tree = parser.parse(code_snippet, None).unwrap();
    let root_node = tree.root_node();

    let mut results: Vec<Box<dyn std::any::Any>> = Vec::new();
    let extractors: Vec<&dyn InfoExtractor> = vec![
        &struct_extractor,
        &function_extractor,
        &type_alias_extractor,
        &impl_extractor,
    ];
    // let mut node_kinds: HashSet<String> = HashSet::new();
    traverse::traverse_tree(
        root_node,
        code_snippet,
        &extractors,
        "code_snippet.rs".to_string(),
        &mut results,
        // &mut node_kinds,
        &mut std::collections::HashSet::new(),
    );

    // println!("Unique node kinds (single file): {:?}", node_kinds);

    for result in results {
        if let Some(struct_info) = result.downcast_ref::<StructInfo>() {
            println!("  Found struct: {:?}", struct_info);
        } else if let Some(function_info) = result.downcast_ref::<FunctionInfo>() {
            println!("  Found function: {:?}", function_info);
        } else if let Some(type_alias_info) = result.downcast_ref::<TypeAliasInfo>() {
            println!("  Found type alias: {:?}", type_alias_info);
        } else if let Some(impl_info) = result.downcast_ref::<ImplInfo>() {
            println!("  Found impl: {:?}", impl_info);
        } else {
            println!("  Unknown type of info extracted");
        }
    }

    println!("Single file parsing complete.");
    Ok(())
}
