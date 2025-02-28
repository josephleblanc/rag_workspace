use std::collections::HashMap;
use std::collections::HashSet;
use std::{any::Any, fs, path::Path};

use tree_sitter::{Node, Parser};
use walkdir::WalkDir;

use anyhow::{Context, Result};
use crate::debug::maybe_collect_node_kind;

// Define a trait for extraction
pub trait InfoExtractor {
    fn extract(&self, node: Node, code: &str, file_path: String) -> Option<Box<dyn Any>>;
    fn node_kind(&self) -> &'static str;
}

pub fn traverse_tree(
    node: Node,
    code: &str,
    extractors: &[&dyn InfoExtractor], // Use a slice of trait objects
    file_path: String,
    results: &mut Vec<Box<dyn Any>>,  // Store the results
    node_kinds: &mut HashSet<String>, // Collect node kinds
    ) {

    // Recursively traverse children, but only if the current node wasn't already extracted
    // This prevents us from recursing too deeply after we've found a struct, function, etc.
    let mut extracted = false;
    for extractor in extractors {
        if node.kind() == extractor.node_kind() {
            extract_results(node, code, extractors, &file_path, results);
            extracted = true;
            break;
        }
    }

    if !extracted {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                extract_results(node, code, extractors, &file_path, results);
                traverse_tree(
                    cursor.node(),
                    code,
                    extractors,
                    file_path.clone(),
                    results,
                    node_kinds,
                );
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}

fn extract_results(
    node: Node<'_>,
    code: &str,
    extractors: &[&dyn InfoExtractor],
    file_path: &String,
    results: &mut Vec<Box<dyn Any>>,
) {
    for extractor in extractors {
        if node.kind() == extractor.node_kind() {
            if let Some(info) = extractor.extract(node, code, file_path.clone()) {
                // Store the extracted info
                results.push(info);
            }
        }
    }
}

pub fn traverse_and_count_node_kinds(
    root_dir: &Path,
    ignored_directories: Option<Vec<String>>,
    _extractors: Vec<&dyn InfoExtractor>,
) -> Result<HashMap<String, usize>> {
    let mut node_kind_counts: HashMap<String, usize> = HashMap::new();

    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let entry_name = entry.file_name().to_string_lossy();

        if let Some(ref ignore_list) = ignored_directories {
            if entry.depth() > 0
                && entry.file_type().is_dir()
                && ignore_list.contains(&entry_name.to_string())
            {
                continue;
            }
        }

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let code = fs::read_to_string(path)?;
            let mut parser = Parser::new();
            parser
                .set_language(&tree_sitter_rust::LANGUAGE.into())
                .context("Error loading Rust grammar")?;
            let tree = parser.parse(&code, None).unwrap();
            let root_node = tree.root_node();
            let mut node_kinds: HashSet<String> = HashSet::new();
            traverse_tree(
                root_node,
                &code,
                &[],
                "".to_string(),
                &mut Vec::new(),
                &mut node_kinds,
            );
            node_kinds
                .iter()
                .for_each(|kind| *node_kind_counts.entry(kind.clone()).or_insert(0) += 1);
        }
    }
    Ok(node_kind_counts)
}

pub fn traverse_and_parse_directory(
    root_dir: &Path,
    ignored_directories: Option<Vec<String>>,
    extractors: Vec<&dyn InfoExtractor>,
) -> Result<Vec<Box<dyn Any>>> {
    let mut all_results: Vec<Box<dyn Any>> = Vec::new();

    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let entry_name = entry.file_name().to_string_lossy();

        if let Some(ref ignore_list) = ignored_directories {
            if entry.depth() > 0
                && entry.file_type().is_dir()
                && ignore_list.contains(&entry_name.to_string())
            {
                println!("Ignoring directory: {}", path.display());
                continue;
            }
        }

        if path.is_file() {
            if path.extension().map_or(false, |ext| ext == "rs") {
                println!("Parsing file: {}", path.display());
                let code = fs::read_to_string(path)
                    .with_context(|| format!("Failed to read file '{}'", path.display()))?;

                let mut parser = Parser::new();
                parser
                    .set_language(&tree_sitter_rust::LANGUAGE.into())
                    .context("Error loading Rust grammar")?;
                let tree = parser.parse(&code, None);

                match tree {
                    Some(syntax_tree) => {
                        // Convert the relative path to an absolute path
                        let absolute_path = path.canonicalize().with_context(|| {
                            format!("Failed to canonicalize path: {}", path.display())
                        })?;
                        let root_node = syntax_tree.root_node();
                        let mut results: Vec<Box<dyn Any>> = Vec::new();
                        let mut node_kinds: HashSet<String> = HashSet::new();
                        traverse_tree(
                            root_node,
                            &code,
                            extractors.as_slice(),
                            absolute_path.display().to_string(),
                            &mut results,
                            &mut node_kinds,
                        );
                        println!("Unique node kinds: {:?}", node_kinds);
                        all_results.extend(results);
                    }
                    None => {
                        println!("Parsing failed for file: {}", path.display());
                    }
                }
            }
        }
    }
    Ok(all_results)
}
