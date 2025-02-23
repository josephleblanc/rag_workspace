use tree_sitter::Node;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImplInfo {
    pub name: String,
    pub is_pub: bool,
    pub start_position: usize,
    pub end_position: usize,
    pub file_path: String,
}

impl Default for ImplInfo {
    fn default() -> Self {
        ImplInfo {
            name: String::new(),
            is_pub: false,
            start_position: 0,
            end_position: 0,
            file_path: String::new(),
        }
    }
}

pub fn extract_impl_info(impl_node: Node<'_>, source_code: &str, file_path: String) -> ImplInfo {
    let mut impl_info = ImplInfo::default();

    impl_info.start_position = impl_node.start_byte();
    impl_info.end_position = impl_node.end_byte();
    impl_info.file_path = file_path.to_string();

    // Extract the name of the type being implemented
    if let Some(type_node) = impl_node.child_by_field_name("type") {
        impl_info.name = type_node
            .utf8_text(source_code.as_bytes())
            .unwrap()
            .to_string();
    }

    // Check for visibility (pub keyword)
    if let Some(visibility_node) = impl_node.child_by_field_name("visibility") {
        if visibility_node.kind() == "pub" {
            impl_info.is_pub = true;
        }
    }

    impl_info
}
