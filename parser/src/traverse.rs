// In your lib.rs or main.rs

use std::fs;
use std::path::Path;
use tree_sitter::{Parser, TreeCursor};
use walkdir::WalkDir;

use crate::extract::{extract_struct_info, StructInfo};

pub fn traverse_and_parse_directory(
    root_dir: &Path,
    ignored_directories: Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
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
                        // Now using safe Rust API of tree-sitter for parsing
                        let mut parser = Parser::new();
                        parser
                            .set_language(&tree_sitter_rust::LANGUAGE.into())
                            .expect("Error loading Rust grammar");
                        let tree = parser.parse(&code, None); // Safe Rust API call

                        match tree {
                            Some(syntax_tree) => {
                                println!("Successfully parsed file content.");
                                // --- Start of tree traversal and struct extraction ---
                                let root_node = syntax_tree.root_node();
                                let mut cursor = root_node.walk();

                                loop {
                                    let current_node = cursor.node();

                                    if current_node.kind() == "struct_item" {
                                        println!("    Found struct_item node!");
                                        let struct_info = extract_struct_info(current_node, &code);
                                        println!("    Extracted Struct: {:?}", struct_info);

                                        // --- Print start and end positions ---
                                        println!(
                                            "    Start Position: {}, End Position: {}",
                                            struct_info.start_position, struct_info.end_position
                                        );

                                        // --- NEW: Print the code snippet ---
                                        let struct_definition_code = &code
                                            [struct_info.start_position..struct_info.end_position];
                                        println!("    Code Snippet:\n{}", struct_definition_code);
                                        println!("    --- End Code Snippet ---");

                                        // --- Print field information --- (unchanged)
                                        if !struct_info.fields.is_empty() {
                                            println!("    Fields:");
                                            for field in &struct_info.fields {
                                                println!(
                                                    "      - Name: {}, Type: {}, Pub: {}",
                                                    field.name, field.type_name, field.is_pub
                                                );
                                            }
                                        }
                                        // --- End print field information ---
                                    }

                                    if cursor.goto_first_child() {
                                        continue;
                                    }
                                    while !cursor.goto_next_sibling() {
                                        if !cursor.goto_parent() {
                                            break;
                                        }
                                    }
                                    if cursor.node().kind() == "source_file" {
                                        break;
                                    }
                                }
                                // --- End of tree traversal and struct extraction ---
                            }
                            None => {
                                eprintln!("Parsing failed for file: {}", path.display());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading file '{}': {}", path.display(), e);
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Corrected root_directory path ---
    let root_directory = Path::new("./example_traverse_target/src"); // Corrected path

    let directories_to_ignore = Some(vec!["examples".to_string(), "assets".to_string()]);
    traverse_and_parse_directory(root_directory, directories_to_ignore)?;

    // --- Removed code snippet parsing ---

    println!("Directory traversal and parsing complete."); // Updated message
    Ok(())
}
