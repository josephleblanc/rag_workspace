// In your lib.rs or main.rs

use crate::function_extractor::extract_function_info;
use crate::struct_extractor::extract_struct_info;
use std::{any::Any, fs, path::Path};
use tree_sitter::{Node, Parser};
use walkdir::WalkDir;

// Define a trait for extraction
pub trait InfoExtractor {
    fn extract(&self, node: Node, code: &str) -> Option<Box<dyn Any>>;
    fn node_kind(&self) -> &'static str; // Add a method to identify the node kind
}

// Generic traversal function
pub fn traverse_tree(
    node: Node,
    code: &str,
    extractors: &[&dyn InfoExtractor], // Use a slice of trait objects
    results: &mut Vec<Box<dyn Any>>, // Store the results
) {
    // Print the node kind for debugging
    println!("Node Kind: {}", node.kind());

    // Check if any extractor matches the current node
    for extractor in extractors {
        if node.kind() == extractor.node_kind() {
            if let Some(info) = extractor.extract(node, code) {
                // Store the extracted info
                results.push(info);
            }
        }
    }

    // Recursively traverse children
    let mut cursor = node.walk();    
    if cursor.goto_first_child() {
        loop {
            traverse_tree(cursor.node(), code, extractors, results);
            if !cursor.goto_next_sibling() { break; }
        }
    }
}

pub fn traverse_and_parse_directory(
    root_dir: &Path,
    ignored_directories: Option<Vec<String>>,
    extractors: Vec<&dyn InfoExtractor>, // Take a Vec of trait objects
) -> Result<Vec<Box<dyn Any>>, Box<dyn std::error::Error>> {
    let mut all_results: Vec<Box<dyn Any>> = Vec::new();

    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let entry_name = entry.file_name().to_string_lossy();

        if let Some(ref ignore_list) = ignored_directories {
            if entry.depth() > 0
                && entry.file_type().is_dir()
                && ignore_list.contains(&entry_name.to_string())
            {
                println!("Ignoring directory: {}", path.display());
                continue;
            }
        }

        if path.is_file() {
            if path.extension().map_or(false, |ext| ext == "rs") {
                println!("Parsing file: {}", path.display());
                match fs::read_to_string(path) {
                    Ok(code) => {
                        let mut parser = Parser::new();
                        if let Err(e) = parser.set_language(&tree_sitter_rust::LANGUAGE.into()) {
                            println!("Error loading Rust grammar: {}", e);
                            continue;
                        }
                        let tree = parser.parse(&code, None);

                        match tree {
                            Some(syntax_tree) => {
                                let root_node = syntax_tree.root_node();
                                let mut results: Vec<Box<dyn Any>> = Vec::new(); // Results for this file
                                traverse_tree(root_node, &code, &extractors, &mut results);
                                all_results.extend(results); // Accumulate results from all files
                            }
                            None => {
                                println!("Parsing failed for file: {}", path.display());
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error reading file '{}': {}", path.display(), e);
                    }
                }
            }
        }
    }
    Ok(all_results) // Return the accumulated results
}

// Example implementations for StructInfo and FunctionInfo extractors
use crate::struct_extractor::StructInfo;
use crate::function_extractor::FunctionInfo;

pub struct StructInfoExtractor {}

impl InfoExtractor for StructInfoExtractor {
    fn extract(&self, node: Node, code: &str) -> Option<Box<dyn Any>> {
        if node.kind() == "struct_item" {
            Some(Box::new(extract_struct_info(node, code)))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "struct_item"
    }
}

pub struct FunctionInfoExtractor {}

impl InfoExtractor for FunctionInfoExtractor {
    fn extract(&self, node: Node, code: &str) -> Option<Box<dyn Any>> {
        if node.kind() == "function_item" {
            Some(Box::new(extract_function_info(node, code)))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "function_item"
    }
}
