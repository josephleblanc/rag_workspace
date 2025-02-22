#[derive(Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<(String, String)>, // (name, type)
    pub return_type: Option<String>,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
}

pub fn extract_function_info(node: tree_sitter::Node, code: &str) -> FunctionInfo {
    let mut name = String::new();
    let mut parameters: Vec<(String, String)> = Vec::new();
    let mut return_type: Option<String> = None;
    let mut is_pub = false;

    let start_position = node.start_byte();
    let end_position = node.end_byte();

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "visibility_modifier" => {
                is_pub = true;
            }
            "identifier" => {
                name = child.utf8_text(code.as_bytes()).unwrap().to_string();
            }
            "parameters" => {
                let mut param_cursor = child.walk();
                for param in child.children(&mut param_cursor) {
                    if param.kind() == "parameter" {
                        let mut param_name = String::new();
                        let mut param_type = String::new();
                        let mut type_found = false;

                        let mut param_child_cursor = param.walk();
                        for param_child in param.children(&mut param_child_cursor) {
                            match param_child.kind() {
                                "identifier" => {
                                    param_name = param_child.utf8_text(code.as_bytes()).unwrap().to_string();
                                }
                                "type_identifier" => {
                                    param_type = param_child.utf8_text(code.as_bytes()).unwrap().to_string();
                                    type_found = true;
                                }
                                _ => {}
                            }
                        }
                        if type_found {
                            parameters.push((param_name, param_type));
                        } else {
                            parameters.push((param_name, String::new()));
                        }
                    }
                }
            }
            "return_type" => {
                let type_node = child.child(0).unwrap();
                return_type = Some(type_node.utf8_text(code.as_bytes()).unwrap().to_string());
            }
            _ => {}
        }
    }

    FunctionInfo {
        name,
        parameters,
        return_type,
        is_pub,
        start_position,
        end_position,
    }
}
```

traverse.rs
```rust
<<<<<<< SEARCH
                                    if current_node.kind() == "struct_item" {
                                        println!("    Found struct_item node!");
                                        let struct_info = extract_struct_info(current_node, &code);
                                        println!("    Extracted Struct: {:?}", struct_info);
                                    }
```
