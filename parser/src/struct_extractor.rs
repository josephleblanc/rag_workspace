use tree_sitter::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub is_pub: bool,
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct StructInfo {
    pub name: String,
    pub is_pub: bool,
    pub doc_comment: Option<String>, // Keeping doc_comment as Option<String> for now, can change to Vec<String> if needed for multiple doc comments
    pub attributes: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub start_position: usize,
    pub end_position: usize,
}

pub fn extract_struct_info(struct_node: Node<'_>, source_code: &str) -> StructInfo {
    let mut struct_info = StructInfo::default();

    // Initialize end position (end position remains the same - end of struct definition)
    struct_info.end_position = struct_node.end_byte();

    // 1. Determine Start Position and Extract Doc Comments & Attributes (preceding siblings)
    let mut start_position = struct_node.start_byte(); // Initial start position is struct keyword
    let mut current_sibling = struct_node.prev_sibling();

    while let Some(sibling) = current_sibling {
        match sibling.kind() {
            "line_comment" | "block_comment" => {
                let comment_text = sibling
                    .utf8_text(source_code.as_bytes())
                    .unwrap()
                    .trim()
                    .to_string();
                if struct_info.doc_comment.is_none() {
                    // Currently storing only the *first* doc comment encountered (closest to struct)
                    struct_info.doc_comment = Some(comment_text);
                }
                start_position = sibling.start_byte(); // Move start position backwards
            }
            "attribute_item" => {
                let attribute_text = sibling
                    .utf8_text(source_code.as_bytes())
                    .unwrap()
                    .trim()
                    .to_string();
                struct_info.attributes.insert(0, attribute_text); // Add attributes to the front to maintain order
                start_position = sibling.start_byte(); // Move start position backwards
            }
            _ => break, // Stop if not a comment or attribute
        }
        current_sibling = sibling.prev_sibling();
    }
    struct_info.start_position = start_position; // Final start position after checking preceding siblings

    // 2. Check for visibility (pub) - Revised method: Iterate children and check kind (same as before)
    let mut cursor = struct_node.walk();
    if cursor.goto_first_child() {
        loop {
            let child_node = cursor.node();
            if child_node.kind() == "visibility_modifier" {
                struct_info.is_pub = true;
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    // 3. Extract struct name (should still work - same as before)
    if let Some(name_node) = struct_node.child_by_field_name("name") {
        struct_info.name = name_node
            .utf8_text(source_code.as_bytes())
            .unwrap()
            .to_string();
    }

    // 4. Extract fields (should still work - same as before)
    if let Some(field_list_node) = struct_node.child_by_field_name("body") {
        let mut field_cursor = field_list_node.walk();
        if field_cursor.goto_first_child() {
            loop {
                let field_node = field_cursor.node();
                if field_node.kind() == "field_declaration" {
                    let field_info = extract_field_info(field_node, source_code);
                    struct_info.fields.push(field_info);
                }
                if !field_cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    struct_info
}

fn extract_field_info(field_node: Node<'_>, source_code: &str) -> FieldInfo {
    let mut field_info = FieldInfo {
        name: String::new(),
        type_name: String::new(),
        is_pub: false,
    };

    // 1. Check for field visibility (pub) (same as before)
    let mut cursor = field_node.walk();
    if cursor.goto_first_child() {
        loop {
            let child_node = cursor.node();
            if child_node.kind() == "visibility_modifier" {
                field_info.is_pub = true;
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    // 2. Extract field name (same as before)
    if let Some(name_node) = field_node.child_by_field_name("name") {
        field_info.name = name_node
            .utf8_text(source_code.as_bytes())
            .unwrap()
            .to_string();
    }

    // 3. Extract field type (same as before)
    if let Some(type_node) = field_node.child_by_field_name("type") {
        field_info.type_name = type_node
            .utf8_text(source_code.as_bytes())
            .unwrap()
            .to_string();
    }

    field_info
}
