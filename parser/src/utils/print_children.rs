#![cfg(feature = "print_children_struct")]
use tree_sitter::Node;

pub(crate) fn print_struct_item(node: Node<'_>) {
    println!("Found a struct_item");
    println!("  start_byte: {}", node.start_byte());
    println!("  end_byte: {}", node.end_byte());
}

pub(crate) fn print_children_struct(child: Node<'_>) {
    println!("  Child kind: {}", child.kind());
    println!("    start_byte: {}", child.start_byte());
    println!("    end_byte: {}", child.end_byte());
}
