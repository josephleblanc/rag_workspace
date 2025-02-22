// src/main.rs
use std::fs;
use std::path::Path;
use tree_sitter::{Language, Parser, TreeCursor};
use walkdir::WalkDir;

mod extract;
use extract::{extract_struct_info, StructInfo};
mod debug; // Import the debug module
mod traverse;
use traverse::traverse_and_parse_directory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_directory = Path::new("../example_traverse_target/src");

    let directories_to_ignore = Some(vec!["examples".to_string(), "assets".to_string()]);
    traverse_and_parse_directory(root_directory, directories_to_ignore)?; // Removed call to directory traversal

    // --- Start of new code for single file parsing ---
    let code_snippet = r#"
        /// A [`Handle`] to the [`AnimationGraph`] to be used by the [`AnimationPlayer`](crate::AnimationPlayer) on the same entity.
        #[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, From)]
        #[reflect(Component, Default)]
        pub struct AnimationGraphHandle(pub Handle<AnimationGraph>);

        struct AnotherStruct {
            field1: i32,
        }
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    let tree = parser.parse(code_snippet, None).unwrap();
    let root_node = tree.root_node();

    // println!("\n--- Syntax Tree ---"); // Separator for clarity
    // debug::print_syntax_tree(root_node, code_snippet, 0); // Call debug print function
    // println!("\n--- End Syntax Tree ---"); // Separator for clarity

    let mut cursor = root_node.walk();

    loop {
        let current_node = cursor.node();

        if current_node.kind() == "struct_item" {
            println!("Found struct_item node!");
            let struct_info = extract_struct_info(current_node, code_snippet);
            println!("Extracted Struct: {:?}", struct_info);
        }

        // Depth-first traversal: try to go to first child, if not, try next sibling, if not, go to parent's next sibling
        if cursor.goto_first_child() {
            continue; // Go deeper into the first child
        }

        // If no children, try to go to next sibling
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                // If no next sibling, go up to parent
                break; // If no parent (at root), traversal is complete
            }
        }
        if cursor.node().kind() == "source_file" {
            // Added check to break at source_file level
            break; // Stop when we are back at the source_file level without siblings
        }
    }
    // --- End of original traversal loop ---

    println!("Single file parsing complete.");
    Ok(())
}
