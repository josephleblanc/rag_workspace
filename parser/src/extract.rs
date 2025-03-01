use std::any::Any;

// src/extract.rs
use serde::{Deserialize, Serialize};
use tree_sitter::Node;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum EnumVariantType {
    Unit,
    Tuple(Vec<String>), // Store the types of the tuple fields
    Struct(Vec<(String, String)>), // Store field names and types for struct-like variants
    #[default]
    Unspecified,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnumVariantInfo {
    pub name: String,
    pub variant_type: EnumVariantType,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnumInfo {
    pub name: String,
    pub variants: Vec<EnumVariantInfo>,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ExtractedData {
    pub structs: Vec<StructInfo>,
    pub functions: Vec<FunctionInfo<()>>,
    pub type_aliases: Vec<TypeAliasInfo>,
    pub impls: Vec<ImplInfo>,
    pub use_dependencies: Vec<UseDependencyInfo>,
    pub mods: Vec<ModInfo>,
    pub enums: Vec<EnumInfo>,
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
pub struct FunctionInfo<ParameterInfo> {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
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

pub struct EnumInfoExtractor {}

impl InfoExtractor for EnumInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "enum_item" {
            let mut enum_info = EnumInfo {
                name: String::new(),
                variants: Vec::new(),
                is_pub: false,
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
            };

            println!("EnumInfoExtractor::extract - Node kind: {}", node.kind());
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                println!("  Child kind: {}", child.kind());
                match child.kind() {
                    "visibility_modifier" => {
                        enum_info.is_pub = true;
                    }
                    "type_identifier" => {
                        if let Ok(name) = child.utf8_text(code.as_bytes()) {
                            enum_info.name = name.to_string();
                            println!("  Enum name: {}", enum_info.name);
                        }
                    }
                    "enum_body" => {
                        // Extract enum variants here
                        println!("  Found enum_body");
                        let mut variant_cursor = child.walk();
                        for variant in child.children(&mut variant_cursor) {
                            println!("    Variant kind: {}", variant.kind());
                            if variant.kind() == "enum_variant" {
                                let mut variant_info = EnumVariantInfo {
                                    name: String::new(),
                                    variant_type: EnumVariantType::Unit, // Default to Unit
                                };
                                let mut name_cursor = variant.walk();
                                for name_child in variant.children(&mut name_cursor) {
                                    println!("      Name child kind: {}", name_child.kind());
                                    match name_child.kind() {
                                        "identifier" => {
                                            if let Ok(name) = name_child.utf8_text(code.as_bytes())
                                            {
                                                variant_info.name = name.to_string();
                                                println!("      Variant name: {}", variant_info.name);
                                            }
                                        }
                                        "ordered_field_declaration_list" => {
                                            // Handle tuple-like variants
                                            println!("      Found ordered_field_declaration_list");
                                            let mut tuple_fields: Vec<String> = Vec::new();
                                            let mut field_cursor = name_child.walk();
                                            for field in name_child.children(&mut field_cursor) {
                                                println!("        Field kind: {}", field.kind());
                                                if field.kind() == "type" {
                                                    if let Ok(field_type) =
                                                        field.utf8_text(code.as_bytes())
                                                    {
                                                        tuple_fields.push(field_type.to_string());
                                                        println!("        Tuple field type: {}", field_type);
                                                    }
                                                }
                                            }
                                            variant_info.variant_type =
                                                EnumVariantType::Tuple(tuple_fields);
                                        }
                                        "record_field_list" => {
                                            // Handle struct-like variants
                                            println!("      Found record_field_list");
                                            let mut struct_fields: Vec<(String, String)> = Vec::new();
                                            let mut field_cursor = name_child.walk();
                                            for field in name_child.children(&mut field_cursor) {
                                                println!("        Field kind: {}", field.kind());
                                                if field.kind() == "field_declaration" {
                                                    let mut field_name = String::new();
                                                    let mut field_type = String::new();
                                                    let mut field_cursor2 = field.walk();
                                                    for field_child in field.children(&mut field_cursor2) {
                                                        println!("          Field child kind: {}", field_child.kind());
                                                        match field_child.kind() {
                                                            "field_identifier" => {
                                                                if let Ok(name) = field_child.utf8_text(code.as_bytes()) {
                                                                    field_name = name.to_string();
                                                                    println!("          Field name: {}", field_name);
                                                                }
                                                            }
                                                            "type" | "primitive_type" => {
                                                                if let Ok(typ) = field_child.utf8_text(code.as_bytes()) {
                                                                    field_type = typ.to_string();
                                                                    println!("          Field type: {}", field_type);
                                                                }
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                    struct_fields.push((field_name, field_type));
                                                }
                                            }
                                            variant_info.variant_type =
                                                EnumVariantType::Struct(struct_fields);
                                        }
                                        _ => {}
                                    }
                                }
                                enum_info.variants.push(variant_info.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }

            Some(Box::new(enum_info))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "enum_item"
    }
}

pub struct ModInfoExtractor {}

impl InfoExtractor for ModInfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>> {
        if node.kind() == "mod_item" {
            let mut mod_info = ModInfo {
                name: String::new(),
                is_pub: false,
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
            };

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "visibility_modifier" => {
                        mod_info.is_pub = true;
                    }
                    "identifier" => {
                        if let Ok(name) = child.utf8_text(code.as_bytes()) {
                            mod_info.name = name.to_string();
                        }
                    }
                    _ => {}
                }
            }

            Some(Box::new(mod_info))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "mod_item"
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

            extract_use_segments(node, code, &mut use_dependency_info);

            Some(Box::new(use_dependency_info))
        } else {
            None
        }
    }

    fn node_kind(&self) -> &'static str {
        "use_declaration"
    }
}

fn extract_use_segments(node: Node, code: &str, use_dependency_info: &mut UseDependencyInfo) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "scoped_identifier" | "path" | "use_group" | "identifier" => {
                extract_path_segments(child, code, &mut use_dependency_info.segments);
            }
            "use_as_clause" => {
                // Handle the "as" alias
                let mut alias_cursor = child.walk();
                for alias_child in child.children(&mut alias_cursor) {
                    if alias_child.kind() == "identifier"
                        || alias_child.kind() == "scoped_identifier"
                    {
                        if let Ok(alias) = alias_child.utf8_text(code.as_bytes()) {
                            use_dependency_info.alias = Some(alias.to_string());
                            break;
                        }
                    }
                }
            }
            "use_wildcard" => {
                use_dependency_info.segments.push("*".to_string());
            }
            _ => {}
        }
    }
}

fn extract_path_segments(node: Node, code: &str, segments: &mut Vec<String>) {
    match node.kind() {
        "identifier" => {
            if let Ok(segment) = node.utf8_text(code.as_bytes()) {
                segments.push(segment.to_string());
            }
        }
        "scoped_identifier" | "path" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                extract_path_segments(child, code, segments);
            }
        }
        "use_group" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                extract_path_segments(child, code, segments);
            }
        }
        _ => {}
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
                        struct_info
                            .attributes
                            .push(child.utf8_text(code.as_bytes()).unwrap().to_string());
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
            let mut function_info: FunctionInfo<()> = FunctionInfo {
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
