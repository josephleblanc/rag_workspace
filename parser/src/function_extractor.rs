use tree_sitter::Node;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<(String, String)>,
    pub return_type: Option<String>,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
}

pub fn extract_function_info(node: tree_sitter::Node, code: &str) -> FunctionInfo {
    let mut function_info = FunctionInfo::default();

    function_info.start_position = node.start_byte();
    function_info.end_position = node.end_byte();

    let mut cursor = node.walk();

    // Extract function name
    if let Some(name_node) = node.child_by_field_name("name") {
        function_info.name = name_node.utf8_text(code.as_bytes()).unwrap().to_string();
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
                                    name = child_node.utf8_text(code.as_bytes()).unwrap().to_string();
                                }
                                "type_identifier" => {
                                    type_name = child_node.utf8_text(code.as_bytes()).unwrap().to_string();
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
        function_info.return_type =
            Some(return_type_node.utf8_text(code.as_bytes()).unwrap().to_string());
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

    function_info
}
