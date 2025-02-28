use std::any::Any;

// src/extract.rs
use serde::{Deserialize, Serialize};
use tree_sitter::Node;

use crate::traverse::InfoExtractor;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub is_pub: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StructInfo {
    pub name: String,
    pub is_pub: bool,
    pub doc_comment: Option<String>, // Keeping doc_comment as Option<String> for now, can change to Vec<String> if needed for multiple doc comments
    pub attributes: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExtractedData {
    pub structs: Vec<StructInfo>,
    pub functions: Vec<FunctionInfo>,
    pub type_aliases: Vec<TypeAliasInfo>,
    pub impls: Vec<ImplInfo>,
    pub use_dependencies: Vec<UseDependencyInfo>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImplInfo {
    pub name: String,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UseDependencyInfo {
    pub segments: Vec<String>,
    pub alias: Option<String>,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TypeAliasInfo {
    pub name: String,
    pub aliased_type: String,
    pub is_pub: bool,
    pub attributes: Vec<String>,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<(String, String)>,
    pub return_type: Option<String>,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

pub struct ImplInfoExtractor {}

impl InfoExtractor for ImplInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "impl_item" {
            let mut cursor = node.walk();
            let mut impl_info = ImplInfo {
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
                ..Default::default()
            };
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "visibility_modifier" => {
                        impl_info.is_pub = true;
                    }
                    "type_identifier" => {
                        impl_info.name = child.utf8_text(code.as_bytes()).unwrap().to_string();
                    }
                    _ => {}
                }
            }
            Some(Box::new(impl_info))
        } else {
            None
        }
        // TODO: Restructure the downcaste in main.rs to try getting this to work again.
        // The current problem is that the downcaste is from the `Any` type and does not
        // correctly downcast into the `ImplInfo` type.
        // if node.kind() == "impl_item" {
        //     match crate::impl_extractor::extract_impl_info(node, code, file_path) {
        //         Ok(impl_info) => Some(Box::new(impl_info)),
        //         Err(e) => {
        //             eprintln!("Failed to extract impl info: {}", e);
        //             None // Or handle the error as appropriate for your application
        //         }
        //     }
        // } else {
        //     None
        // } [ #someday ]
    }

    fn node_kind(&self) -> &'static str {
        "impl_item"
    }
}

pub struct UseDependencyInfoExtractor {}

impl InfoExtractor for UseDependencyInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "use_declaration" {
            let mut use_dependency_info = UseDependencyInfo {
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
                ..Default::default()
            };

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "visibility_modifier" => {
                        use_dependency_info.is_pub = true;
                    }
                    "scoped_identifier" | "path" => {
                        // Extract the path segments
                        extract_path_segments(child, code, &mut use_dependency_info.segments);
                    }
                    "scoped_use_list" => {
                        let mut list_cursor = child.walk();
                        for list_child in child.children(&mut list_cursor) {
                            match list_child.kind() {
                                "path" => {
                                    extract_path_segments(list_child, code, &mut use_dependency_info.segments);
                                }
                                "use_list" => {
                                    let mut use_list_cursor = list_child.walk();
                                    for use_list_child in list_child.children(&mut use_list_cursor) {
                                        match use_list_child.kind() {
                                            "scoped_identifier" | "identifier" => {
                                                extract_path_segments(use_list_child, code, &mut use_dependency_info.segments);
                                            }
                                            "use_wildcard" => {
                                                // Handle wildcard imports (e.g., `*`)
                                                use_dependency_info.segments.push("*".to_string());
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    "use_as_clause" => {
                        // Handle the "as" alias
                        let mut alias_cursor = child.walk();
                        for alias_child in child.children(&mut alias_cursor) {
                            if alias_child.kind() == "identifier" {
                                use_dependency_info.alias =
                                    Some(alias_child.utf8_text(code.as_bytes()).unwrap().to_string());
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }

            Some(Box::new(use_dependency_info))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "use_declaration"
    }
}

fn extract_path_segments(node: Node, code: &str, segments: &mut Vec<String>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "identifier" => {
                segments.push(child.utf8_text(code.as_bytes()).unwrap().to_string());
            }
            "scoped_identifier" => {
                extract_path_segments(child, code, segments);
            }
            _ => {}
        }
    }
}

pub struct TypeAliasInfoExtractor {}

impl InfoExtractor for TypeAliasInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "type_item" {
            let mut type_alias_info = TypeAliasInfo {
                name: String::new(),
                aliased_type: String::new(),
                is_pub: false,
                attributes: Vec::new(),
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
            };

            let mut cursor = node.walk();

            for child in node.children(&mut cursor) {
                match child.kind() {
                    "visibility_modifier" => {
                        type_alias_info.is_pub = true;
                    }
                    "type_identifier" => {
                        type_alias_info.name =
                            child.utf8_text(code.as_bytes()).unwrap().to_string();
                    }
                    "type" => {
                        type_alias_info.aliased_type =
                            child.utf8_text(code.as_bytes()).unwrap().to_string();
                    }
                    "attribute" => {
                        type_alias_info
                            .attributes
                            .push(child.utf8_text(code.as_bytes()).unwrap().to_string());
                    }
                    _ => {}
                }
            }

            Some(Box::new(type_alias_info))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "type_item"
    }
}

pub struct StructInfoExtractor {}

impl InfoExtractor for StructInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "struct_item" {
            let mut cursor = node.walk();
            let mut struct_info = StructInfo {
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
                ..Default::default()
            };
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "visibility_modifier" => {
                        struct_info.is_pub = true;
                    }
                    "type_identifier" => {
                        struct_info.name = child.utf8_text(code.as_bytes()).unwrap().to_string();
                    }
                    "block" => {
                        // handle block - not relevant for struct definition itself
                    }
                    "attribute" => {
                        struct_info
                            .attributes
                            .push(child.utf8_text(code.as_bytes()).unwrap().to_string());
                    }
                    "attribute_item" => {
                        struct_info.attributes.push(child.utf8_text(code.as_bytes()).unwrap().to_string());
                    }
                    _ => {}
                }
            }
            // println!("Extracting struct: {}", struct_info.name);
            Some(Box::new(struct_info))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "struct_item"
    }
}

pub struct FunctionInfoExtractor {}

impl InfoExtractor for FunctionInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "function_item" {
            let mut cursor = node.walk();
            let mut function_info = FunctionInfo {
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
                ..Default::default()
            };
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "visibility_modifier" => {
                        function_info.is_pub = true;
                    }
                    "type_identifier" => {
                        function_info.name = child.utf8_text(code.as_bytes()).unwrap().to_string();
                    }
                    // more here
                    _ => {}
                }
            }
            // println!("Extracting function: {}", struct_info.name);
            Some(Box::new(function_info))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "function_item"
    }
}
