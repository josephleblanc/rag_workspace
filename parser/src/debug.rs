// src/debug.rs
use tree_sitter::Node;

// Recursive function to print the syntax tree
pub fn print_syntax_tree(node: Node, source_code: &str, indent: usize) {
    let node_kind = node.kind();
    let node_text = node.utf8_text(source_code.as_bytes()).unwrap();

    // Indentation for tree-like output
    let indent_str = "  ".repeat(indent);

    println!("{}- Kind: {}, Text: {:?}", indent_str, node_kind, node_text);

    // Recursively print children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            print_syntax_tree(cursor.node(), source_code, indent + 1);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}
