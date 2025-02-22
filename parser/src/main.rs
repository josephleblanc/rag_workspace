use std::fs;
use std::path::Path;
use tree_sitter::{Language, Parser};
use walkdir::WalkDir;

fn traverse_and_parse_directory(
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
                        let tree = parser.parse(code, None); // Safe Rust API call

                        match tree {
                            Some(_syntax_tree) => {
                                println!("Successfully parsed file content. (Tree processing would go here)");
                                // You can now work with the syntax_tree in safe Rust
                                // Example:  syntax_tree.root_node().kind(); // Safe Rust API
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
    let root_directory = Path::new("./path/to/your/bevy/codebase"); // Replace

    let directories_to_ignore = Some(vec!["examples".to_string(), "assets".to_string()]);
    traverse_and_parse_directory(root_directory, directories_to_ignore)?; // Pass language as reference

    println!("Directory traversal and parsing complete.");
    Ok(())
}
