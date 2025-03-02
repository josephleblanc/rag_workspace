#![cfg(feature = "print_blocks")]
use crate::extract::{ExtractedData, FunctionInfo};
use crate::{EnumInfo, ImplInfo, MacroInfo, ModInfo, StructInfo, TypeAliasInfo, UseDependencyInfo};

pub(crate) trait PrintBlock {
    fn print_block(&self, extracted_data: &ExtractedData) -> String;
}

pub(crate) fn print_single_block(extracted_data: &ExtractedData) {
    println!("--- Printing one of each type ---");

    if let Some(item) = extracted_data.structs.first() {
        println!("Struct: {}", item.print_block(&extracted_data));
    } else {
        println!("No structs found.");
    }

    if let Some(item) = extracted_data.functions.first() {
        println!("Function: {}", item.print_block(&extracted_data));
    } else {
        println!("No functions found.");
    }

    if let Some(item) = extracted_data.type_aliases.first() {
        println!("Type Alias: {}", item.print_block(&extracted_data));
    } else {
        println!("No type aliases found.");
    }

    if let Some(item) = extracted_data.impls.first() {
        println!("Impl: {}", item.print_block(&extracted_data));
    } else {
        println!("No impls found.");
    }

    if let Some(item) = extracted_data.use_dependencies.first() {
        println!("Use Dependency: {}", item.print_block(&extracted_data));
    } else {
        println!("No use dependencies found.");
    }

    if let Some(item) = extracted_data.mods.first() {
        println!("Mod: {}", item.print_block(&extracted_data));
    } else {
        println!("No mods found.");
    }

    if let Some(item) = extracted_data.enums.first() {
        println!("Enum: {}", item.print_block(&extracted_data));
    } else {
        println!("No enums found.");
    }

    if let Some(item) = extracted_data.macros.first() {
        println!("Macro: {}", item.print_block(&extracted_data));
    } else {
        println!("No macros found.");
    }
}

pub(crate) fn print_blocks(extracted_data: &ExtractedData) {
    println!("--- Printing all extracted data ---");

    println!("--- Structs ---");
    for item in &extracted_data.structs {
        println!("Struct: {}", item.print_block(&extracted_data));
        println!("---");
    }

    println!("--- Functions ---");
    for item in &extracted_data.functions {
        println!("Function: {}", item.print_block(&extracted_data));
        println!("---");
    }

    println!("--- Type Aliases ---");
    for item in &extracted_data.type_aliases {
        println!("Type Alias: {}", item.print_block(&extracted_data));
        println!("---");
    }

    println!("--- Impls ---");
    for item in &extracted_data.impls {
        println!("Impl: {}", item.print_block(&extracted_data));
        println!("---");
    }

    println!("--- Use Dependencies ---");
    for item in &extracted_data.use_dependencies {
        println!("Use Dependency: {}", item.print_block(&extracted_data));
        println!("---");
    }

    println!("--- Mods ---");
    for item in &extracted_data.mods {
        println!("Mod: {}", item.print_block(&extracted_data));
        println!("---");
    }

    println!("--- Enums ---");
    for item in &extracted_data.enums {
        println!("Enum: {}", item.print_block(&extracted_data));
        println!("---");
    }

    println!("--- Macros ---");
    for item in &extracted_data.macros {
        println!("Macro: {}", item.print_block(&extracted_data));
        println!("---");
    }
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

// #[cfg(feature = "print_blocks")]
impl PrintBlock for FunctionInfo {
    fn print_block(&self, extracted_data: &ExtractedData) -> String {
        let code = extracted_data.file_contents.get(&self.file_path).unwrap();
        code[self.start_position..self.end_position].to_string()
    }
}
