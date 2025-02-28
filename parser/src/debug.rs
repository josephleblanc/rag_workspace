use crate::extract::ImplInfo;
// src/debug.rs
use crate::Any;
use std::any::type_name_of_val;
use tree_sitter::Node;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub(crate) fn process_any_debug(boxed_any: &Box<dyn Any>) {
    println!("Type inside Box<dyn Any>: {}", type_name_of_val(boxed_any));

    if let Some(impl_info) = (&*boxed_any).downcast_ref::<ImplInfo>() {
        println!("Found ImplInfo: {:?}", impl_info);
    } else {
        println!("Unknown type inside Box<dyn Any>");
    }
}

#[allow(dead_code)]
pub fn process_box_take_ownership(boxed_any: Box<dyn Any>) {
    let none: Option<ImplInfo> = None;
    println!(
        "-- Type process_box_take_ownership: {:?}",
        boxed_any.type_id()
    );
    println!("-- Type None id: {:?}", none.type_id());
    match boxed_any.downcast::<ImplInfo>() {
        Ok(boxed_data) => {
            println!("Successfully downcasted to Box<ImplInfo>");
            let data: ImplInfo = *boxed_data; // Now you can "unbox" the Box<ImplInfo> in the usual way
            println!("Data: {:?}", data);
            println!("Value inside name field: {}", data.name);
            // boxed_data is dropped here
        }
        Err(_original_box) => {
            println!("Failed to downcast to Box<MyData>");
            // original_box is the Box<dyn Any> back, you still own it.
            // You might try to downcast to other types or handle it as dyn Any.
        }
    }
    // No box is dropped here explicitly, ownership handled within match arms.
}

#[cfg(feature = "collect_node_kinds")]
pub(crate) fn maybe_collect_node_kind(node: Node, node_kinds: &mut HashSet<String>) {
    node_kinds.insert(node.kind().to_string());
}
