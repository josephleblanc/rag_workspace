// src/extract.rs
use crate::traverse::InfoExtractor;
use crate::utils::print_blocks::PrintBlock;
use serde::{Deserialize, Serialize};
use std::fs;
use tree_sitter::Node;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub type_name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum EnumVariantType {
    // This is pretty much placeholder for now.
    // It will likely get replaced when we add the `syn` crate.
    Unit,
    Tuple(Vec<String>),            // Store the types of the tuple fields
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MacroInfo {
    pub name: String,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

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

impl PrintBlock for EnumInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

impl PrintBlock for ModInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

impl PrintBlock for MacroInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

impl PrintBlock for StructInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

impl PrintBlock for ImplInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

impl PrintBlock for UseDependencyInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

impl PrintBlock for TypeAliasInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

impl PrintBlock for FunctionInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ExtractedData {
    pub structs: Vec<StructInfo>,
    pub functions: Vec<FunctionInfo>,
    pub type_aliases: Vec<TypeAliasInfo>,
    pub impls: Vec<ImplInfo>,
    pub use_dependencies: Vec<UseDependencyInfo>,
    pub mods: Vec<ModInfo>,
    pub enums: Vec<EnumInfo>,
    pub macros: Vec<MacroInfo>,
    pub file_contents: std::collections::HashMap<String, String>,
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
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub is_pub: bool,
    pub is_method: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

pub struct ImplInfoExtractor {}

impl InfoExtractor for ImplInfoExtractor {
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
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
            extracted_data_.impls.push(impl_info);
        }
        Ok(())
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

fn extract_doc_comment(node: Node, code: &str) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "outer_attribute" {
            let text = child.utf8_text(code.as_bytes()).unwrap().to_string();
            if text.starts_with("//!") {
                return Some(text);
            }
        }
    }
    None
}

pub struct MacroInfoExtractor {}

impl InfoExtractor for MacroInfoExtractor {
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
        if node.kind() == "macro_invocation" {
            let mut macro_info = MacroInfo {
                name: String::new(),
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
            };

            // Extract macro name
            if let Some(name_node) = node.child_by_field_name("macro") {
                if let Ok(name) = name_node.utf8_text(code.as_bytes()) {
                    macro_info.name = name.to_string();
                }
            }

            extracted_data_.macros.push(macro_info);
        }
        Ok(())
    }

    fn node_kind(&self) -> &'static str {
        "macro_invocation"
    }
}

pub struct EnumInfoExtractor {}

impl InfoExtractor for EnumInfoExtractor {
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
        if node.kind() == "enum_item" {
            let mut enum_info = EnumInfo {
                name: String::new(),
                variants: Vec::new(),
                is_pub: false,
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
            };

            // Extract enum name
            if let Some(name_node) = node.child_by_field_name("name") {
                if let Ok(name) = name_node.utf8_text(code.as_bytes()) {
                    enum_info.name = name.to_string();
                }
            }

            // Extract enum variants
            if let Some(body_node) = node.child_by_field_name("body") {
                let mut variant_cursor = body_node.walk();
                for variant in body_node.children(&mut variant_cursor) {
                    if variant.kind() == "enum_variant" {
                        extract_enum_variant(variant, code, &mut enum_info);
                    }
                }
            }

            extracted_data_.enums.push(enum_info);
        }
        Ok(())
    }

    fn node_kind(&self) -> &'static str {
        "enum_item"
    }
}

fn extract_enum_variant(node: Node, code: &str, enum_info: &mut EnumInfo) {
    let mut variant_info = EnumVariantInfo {
        name: String::new(),
        variant_type: EnumVariantType::Unit, // Default to Unit
    };
    let mut name_cursor = node.walk();
    for name_child in node.children(&mut name_cursor) {
        match name_child.kind() {
            "identifier" => {
                if let Ok(name) = name_child.utf8_text(code.as_bytes()) {
                    variant_info.name = name.to_string();
                }
            }
            "ordered_field_declaration_list" => {
                // Handle tuple-like variants
                let mut tuple_fields: Vec<String> = Vec::new();
                let mut field_cursor = name_child.walk();
                for field in name_child.children(&mut field_cursor) {
                    if field.kind() == "type" || field.kind() == "primitive_type" {
                        if let Ok(field_type) = field.utf8_text(code.as_bytes()) {
                            tuple_fields.push(field_type.to_string());
                        }
                    }
                }
                variant_info.variant_type = EnumVariantType::Tuple(tuple_fields);
            }
            "record_field_list" => {
                // Handle struct-like variants
                let mut struct_fields: Vec<(String, String)> = Vec::new();
                let mut field_cursor = name_child.walk();
                for field in name_child.children(&mut field_cursor) {
                    if field.kind() == "field_declaration" {
                        let mut field_name = String::new();
                        let mut field_type = String::new();
                        let mut field_cursor2 = field.walk();
                        for field_child in field.children(&mut field_cursor2) {
                            match field_child.kind() {
                                "field_identifier" => {
                                    if let Ok(name) = field_child.utf8_text(code.as_bytes()) {
                                        field_name = name.to_string();
                                    }
                                }
                                "type" | "primitive_type" => {
                                    if let Ok(typ) = field_child.utf8_text(code.as_bytes()) {
                                        field_type = typ.to_string();
                                    }
                                }
                                _ => {}
                            }
                        }
                        struct_fields.push((field_name, field_type));
                    }
                }
                variant_info.variant_type = EnumVariantType::Struct(struct_fields);
            }
            _ => {}
        }
    }
    enum_info.variants.push(variant_info.clone());
}

pub struct ModInfoExtractor {}

impl InfoExtractor for ModInfoExtractor {
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
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

            extracted_data_.mods.push(mod_info);
        }
        Ok(())
    }

    fn node_kind(&self) -> &'static str {
        "mod_item"
    }
}

// TODO: Figure out a better name for this struct.
pub struct UseDependencyInfoExtractor {}

impl InfoExtractor for UseDependencyInfoExtractor {
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
        if node.kind() == "use_declaration" {
            let mut use_dependency_info = UseDependencyInfo {
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
                ..Default::default()
            };

            extract_use_segments(node, code, &mut use_dependency_info);

            extracted_data_.use_dependencies.push(use_dependency_info);
        }
        Ok(())
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
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
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

            extracted_data_.type_aliases.push(type_alias_info);
        }
        Ok(())
    }

    fn node_kind(&self) -> &'static str {
        "type_item"
    }
}

pub struct StructInfoExtractor {}

impl InfoExtractor for StructInfoExtractor {
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
        if node.kind() == "struct_item" {
            println!("Found a struct_item");
            println!("  start_byte: {}", node.start_byte());
            println!("  end_byte: {}", node.end_byte());

            let mut struct_info = StructInfo {
                start_position: node.start_byte(),
                end_position: node.end_byte(), // Initial end_position
                file_path: file_path.to_string(),
                ..Default::default()
            };

            // Extract leading doc comments by iterating backwards through preceding siblings
            let mut current_node = node;
            while let Some(previous_sibling) = current_node.prev_sibling() {
                if previous_sibling.kind() == "line_comment" {
                    let comment_text = previous_sibling.utf8_text(code.as_bytes()).unwrap().trim();
                    if comment_text.starts_with("///") {
                        // Prepend the comment to the existing doc_comment (if any)
                        struct_info.doc_comment = match struct_info.doc_comment {
                            Some(existing_comment) => {
                                Some(format!("{}\n{}", comment_text, existing_comment))
                            }
                            None => Some(comment_text.to_string()),
                        };
                    } else {
                        // If it's not a doc comment, stop searching
                        break;
                    }
                    current_node = previous_sibling; // Move to the previous sibling
                } else {
                    // If it's not a comment, stop searching
                    break;
                }
            }

            let mut cursor = node.walk();
            let mut max_end_byte = node.end_byte(); // Initialize with the node's initial end byte

            for child in node.children(&mut cursor) {
                println!("  Child kind: {}", child.kind());
                println!("    start_byte: {}", child.start_byte());
                println!("    end_byte: {}", child.end_byte());

                max_end_byte = std::cmp::max(max_end_byte, child.end_byte()); // Update max_end_byte

                match child.kind() {
                    "attribute_item" => {
                        if let Ok(attribute) = child.utf8_text(code.as_bytes()) {
                            struct_info.attributes.push(attribute.to_string());
                        }
                    }
                    "visibility_modifier" => {
                        struct_info.is_pub = true;
                    }
                    "type_identifier" => {
                        struct_info.name = child.utf8_text(code.as_bytes()).unwrap().to_string();
                    }
                    "field_declaration_list" => {
                        let mut field_cursor = child.walk();
                        for field in child.children(&mut field_cursor) {
                            if field.kind() == "field_declaration" {
                                let mut field_info = FieldInfo::default();
                                let mut field_cursor2 = field.walk();
                                for field_child in field.children(&mut field_cursor2) {
                                    match field_child.kind() {
                                        "field_identifier" => {
                                            if let Ok(name) = field_child.utf8_text(code.as_bytes())
                                            {
                                                field_info.name = name.to_string();
                                            }
                                        }
                                        "type" => {
                                            if let Ok(typ) = field_child.utf8_text(code.as_bytes())
                                            {
                                                field_info.type_name = typ.to_string();
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                struct_info.fields.push(field_info);
                            }
                        }
                    }
                    "block" => {
                        // handle block - not relevant for struct definition itself
                    }
                    _ => {}
                }
            }
            // Update end_position after processing all children
            // struct_info.end_position = node.end_byte(); // revert to this if the below does not work
            struct_info.end_position = max_end_byte;
            extracted_data_.structs.push(struct_info);
        }
        Ok(())
    }

    fn node_kind(&self) -> &'static str {
        "struct_item"
    }
}

pub struct FunctionInfoExtractor {}

impl InfoExtractor for FunctionInfoExtractor {
    fn extract(
        &self,
        node: Node,
        code: &str,
        file_path: String,
        extracted_data_: &mut ExtractedData,
    ) -> Result<(), anyhow::Error> {
        if node.kind() == "function_item" {
            let mut function_info = FunctionInfo {
                start_position: node.start_byte(),
                end_position: node.end_byte(),
                file_path: file_path.to_string(),
                ..Default::default()
            };
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "visibility_modifier" => {
                        function_info.is_pub = true;
                    }
                    "identifier" => {
                        function_info.name = child.utf8_text(code.as_bytes()).unwrap().to_string();
                    }
                    "parameters" => {
                        if let Some(params_node) = node.child_by_field_name("parameters") {
                            let mut param_cursor = params_node.walk();
                            for param in params_node.children(&mut param_cursor) {
                                if param.kind() == "self_parameter" {
                                    if let Ok(self_param) = param.utf8_text(code.as_bytes()) {
                                        if self_param == "&self"
                                            || self_param == "&mut self"
                                            || self_param == "self"
                                        {
                                            function_info.is_method = true;
                                        }
                                    }
                                }
                                if param.kind() == "parameter" {
                                    let mut param_info = ParameterInfo::default();
                                    let mut param_cursor2 = param.walk();
                                    for param_child in param.children(&mut param_cursor2) {
                                        match param_child.kind() {
                                            "identifier" => {
                                                if let Ok(name) =
                                                    param_child.utf8_text(code.as_bytes())
                                                {
                                                    param_info.name = name.to_string();
                                                }
                                            }
                                            "type" => {
                                                if let Ok(type_name) =
                                                    param_child.utf8_text(code.as_bytes())
                                                {
                                                    param_info.type_name = type_name.to_string();
                                                }
                                            }
                                            "generic_type" => {
                                                if let Ok(type_name) =
                                                    param_child.utf8_text(code.as_bytes())
                                                {
                                                    param_info.type_name = type_name.to_string();
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    function_info.parameters.push(param_info);
                                }
                            }
                        }
                    }
                    "return_type" => {
                        if let Some(return_type_node) = node.child_by_field_name("return_type") {
                            if let Ok(return_type) = return_type_node.utf8_text(code.as_bytes()) {
                                function_info.return_type = Some(return_type.to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
            extracted_data_.functions.push(function_info);
        }
        Ok(())
    }

    fn node_kind(&self) -> &'static str {
        "function_item"
    }
}
