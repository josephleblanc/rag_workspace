// src/main.rs
use std::path::Path;
use tree_sitter::Parser;

mod extract;
mod function_extractor;
mod struct_extractor;
mod traverse;

use function_extractor::FunctionInfo;
use std::collections::HashSet;
use struct_extractor::StructInfo;
use traverse::{
    traverse_and_parse_directory, FunctionInfoExtractor, InfoExtractor, StructInfoExtractor,
};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

// Define a struct to hold all extracted information
#[derive(Serialize, Deserialize, Debug)]
struct ExtractedData {
    structs: Vec<StructInfo>,
    functions: Vec<FunctionInfo>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_directory = Path::new("../example_traverse_target/src");

    let directories_to_ignore = Some(vec!["examples".to_string(), "assets".to_string()]);

    // Create extractors
    let struct_extractor = StructInfoExtractor {};
    let function_extractor = FunctionInfoExtractor {};

    // Collect extractors into a Vec<&dyn InfoExtractor>
    let extractors: Vec<&dyn InfoExtractor> = vec![&struct_extractor, &function_extractor];

    // Traverse the directory and extract information
    let results = traverse_and_parse_directory(root_directory, directories_to_ignore, extractors)?;

    // Process the results
    println!("\n--- Extracted Information ---");
    let mut extracted_data = ExtractedData {
        structs: Vec::new(),
        functions: Vec::new(),
    };

    for result in results {
        if let Some(struct_info) = result.downcast_ref::<StructInfo>() {
            println!("  Found struct: {:?}", struct_info);
            extracted_data.structs.push(struct_info.clone());
        } else if let Some(function_info) = result.downcast_ref::<FunctionInfo>() {
            println!("  Found function: {:?}", function_info);
            extracted_data.functions.push(function_info.clone());
        } else {
            println!("  Unknown type of info extracted");
        }
    }
    println!("--- End Extracted Information ---");

    // Serialize to RON and save to file
    let ron_string = ron::ser::to_string_pretty(&extracted_data, ron::ser::PrettyConfig::default())?;
    let mut file = File::create("extracted_data.ron")?;
    file.write_all(ron_string.as_bytes())?;

    println!("Extracted data saved to extracted_data.ron");

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

        fn hello_world() -> String {
            return "Hello World".to_string();
        }
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    let tree = parser.parse(code_snippet, None).unwrap();
    let root_node = tree.root_node();

    let mut results: Vec<Box<dyn std::any::Any>> = Vec::new();
    let extractors: Vec<&dyn InfoExtractor> = vec![&struct_extractor, &function_extractor]; // Reuse extractors
    let mut node_kinds: HashSet<String> = HashSet::new();
    traverse::traverse_tree(
        root_node,
        code_snippet,
        &extractors,
        "code_snippet.rs".to_string(),
        &mut results,
        &mut node_kinds,
    );

    println!("Unique node kinds (single file): {:?}", node_kinds);

    for result in results {
        if let Some(struct_info) = result.downcast_ref::<StructInfo>() {
            println!("  Found struct: {:?}", struct_info);
        } else if let Some(function_info) = result.downcast_ref::<FunctionInfo>() {
            println!("  Found function: {:?}", function_info);
        } else {
            println!("  Unknown type of info extracted");
        }
    }

    println!("Single file parsing complete.");
    Ok(())
}
