use crate::extract::{ImplInfo, TypeAliasInfo};
use crate::function_extractor::extract_function_info;
use crate::struct_extractor::extract_struct_info;
use crate::impl_extractor::extract_impl_info;
use std::collections::HashSet;
use std::{any::Any, fs, path::Path};

use tree_sitter::{Node, Parser};
use walkdir::WalkDir;

use anyhow::{Context, Result};

// Define a trait for extraction
pub trait InfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>>;
    fn node_kind(&self) -> &'static str;
}

pub fn traverse_tree(
    node: Node,
    code: &str,
    extractors: &[&dyn InfoExtractor], // Use a slice of trait objects
    file_path: String,
    results: &mut Vec<Box<dyn Any>>,  // Store the results
    node_kinds: &mut HashSet<String>, // Collect node kinds
) {
    println!("Node kind: {}", node.kind());
    // Collect node kinds
    node_kinds.insert(node.kind().to_string());

    // Check if any extractor matches the current node
    for extractor in extractors {
        if node.kind() == extractor.node_kind() {
            if let Some(info) = extractor.extract(node, code, file_path.clone()) {
                // Store the extracted info
                results.push(info);
            }
        }
    }

    // Recursively traverse children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            traverse_tree(
                cursor.node(),
                code,
                extractors,
                file_path.clone(),
                results,
                node_kinds,
            );
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    } // Closing brace for if cursor.goto_first_child()
}

fn extract_type_alias_info(node: Node<'_>, source_code: &str, file_path: String) -> Box<dyn Any> {
    let mut type_alias_info = TypeAliasInfo {
        name: String::new(),
        aliased_type: String::new(),
        is_pub: false,
        attributes: Vec::new(),
        start_position: node.start_byte(),
        end_position: node.end_byte(),
        file_path: file_path.to_string(),
    };

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        println!(
            "Child Kind: {}, Text: {:?}",
            child.kind(),
            child.utf8_text(source_code.as_bytes())
        );
        match child.kind() {
            "visibility_modifier" => {
                println!("  Visibility Modifier found");
                type_alias_info.is_pub = true;
            }
            "type_identifier" => {
                println!("  Type Identifier found");
                type_alias_info.name = child.utf8_text(source_code.as_bytes()).unwrap().to_string();
            }
            "type" => {
                println!("  Type found");
                type_alias_info.aliased_type =
                    child.utf8_text(source_code.as_bytes()).unwrap().to_string();
            }
            "attribute" => {
                println!("  Attribute found");
                type_alias_info
                    .attributes
                    .push(child.utf8_text(source_code.as_bytes()).unwrap().to_string());
            }
            _ => {
                println!("  Other kind found: {}", child.kind());
            }
        }
    }

    Box::new(type_alias_info)
}

pub struct ImplInfoExtractor {}

impl InfoExtractor for ImplInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "impl_item" {
            Some(Box::new(extract_impl_info(node, code, file_path)))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "impl_item"
    }
}

pub fn traverse_and_parse_directory(
    root_dir: &Path,
    ignored_directories: Option<Vec<String>>,
    extractors: Vec<&dyn InfoExtractor>, // Take a Vec of trait objects
) -> Result<Vec<Box<dyn Any>>> {
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
                let code = fs::read_to_string(path)
                    .with_context(|| format!("Failed to read file '{}'", path.display()))?;

                let mut parser = Parser::new();
                parser
                    .set_language(&tree_sitter_rust::LANGUAGE.into())
                    .context("Error loading Rust grammar")?;
                let tree = parser.parse(&code, None);

                match tree {
                    Some(syntax_tree) => {
                        // Convert the relative path to an absolute path
                        let absolute_path = path
                            .canonicalize()
                            .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;
                        let root_node = syntax_tree.root_node();
                        let mut results: Vec<Box<dyn Any>> = Vec::new(); // Results for this file
                        let mut node_kinds: HashSet<String> = HashSet::new(); // Collect node kinds
                        traverse_tree(
                            root_node,
                            &code,
                            &extractors,
                            absolute_path.display().to_string(),
                            &mut results,
                            &mut node_kinds,
                        );
                        println!("Unique node kinds: {:?}", node_kinds); // Print node kinds
                        all_results.extend(results); // Accumulate results from all files
                    }
                    None => {
                        println!("Parsing failed for file: {}", path.display());
                    }
                }
            }
        }
    }
    Ok(all_results) // Return the accumulated results
}

// Example implementations for StructInfo and FunctionInfo extractors
pub struct TypeAliasInfoExtractor {}

impl InfoExtractor for TypeAliasInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "type_item" {
            Some(extract_type_alias_info(node, code, file_path))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "type_item"
    }
}

pub struct StructInfoExtractor {}

impl InfoExtractor for StructInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "struct_item" {
            Some(Box::new(extract_struct_info(node, code, file_path)))
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
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "function_item" {
            Some(Box::new(extract_function_info(node, code, file_path)))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "function_item"
    }
}
