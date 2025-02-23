use serde::{Deserialize, Serialize};

use anyhow::Result;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<(String, String)>,
    pub return_type: Option<String>,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

pub fn extract_function_info(
    node: tree_sitter::Node,
    code: &str,
    file_path: String,
) -> Result<FunctionInfo> {
    let mut function_info = FunctionInfo::default();

    function_info.start_position = node.start_byte();
    function_info.end_position = node.end_byte();

    let cursor = node.walk();

    // Extract function name
    if let Some(name_node) = node.child_by_field_name("name") {
        function_info.name = String::from(name_node.utf8_text(code.as_bytes())?);
    }

    // Extract parameters
    if let Some(parameters_node) = node.child_by_field_name("parameters") {
        let mut param_cursor = parameters_node.walk();
        if param_cursor.goto_first_child() {
            loop {
                let param_node = param_cursor.node();
                if param_node.kind() == "parameter" {
                    let mut name = String::new();
                    let mut type_name = String::new();

                    let mut param_child_cursor = param_node.walk();
                    if param_child_cursor.goto_first_child() {
                        loop {
                            let child_node = param_child_cursor.node();
                            match child_node.kind() {
                                "identifier" => {
                                    name = String::from(child_node.utf8_text(code.as_bytes())?);
                                }
                                "type_identifier" => {
                                    type_name = String::from(child_node.utf8_text(code.as_bytes())?);
                                }
                                _ => {}
                            }
                            if !param_child_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    if !name.is_empty() && !type_name.is_empty() {
                        function_info.parameters.push((name, type_name));
                    }
                }
                if !param_cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    // Extract return type
    if let Some(return_type_node) = node.child_by_field_name("return_type") {
        function_info.return_type = Some(String::from(return_type_node.utf8_text(code.as_bytes())?));
    }

    // Check for visibility (pub)
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child_node = cursor.node();
            if child_node.kind() == "visibility_modifier" {
                function_info.is_pub = true;
                break;
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    function_info.file_path = file_path;

    Ok(function_info)
}
