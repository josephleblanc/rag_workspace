use tree_sitter::Node;

#[derive(Debug, Default)]
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
    function_info
}
