// src/extract.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub is_pub: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(dead_code)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(dead_code)]
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
