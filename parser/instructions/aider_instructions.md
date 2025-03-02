# Aider Instructions

While working on different projects it will be good to have a set of
instructions I can copy and paste as needed.

## Project: Parser

This is the `parser` project inside the `rag_worspace` cargo workspace.

### Background

The overall goal of the project is to create a tool to extract chunks of
semantically meaningful data that will later be passed into a database for an
RAG pipeline. The RAG is intended to assist in an LLM's code generation and
refactoring.

The intended database will eventually be a hybrid heterogeneous graph and
vector embedding database. This means it will be important to identify
relationships between data structures. Those relationships will later allow us
to create edges in the graph to represent meaningful connections, and improve
the capability of the RAG and thereby the LLM doing code generation and
refactoring.

The project aims to accept a directory as input, and then parse the `.rs` files in that library for relevant chunks of data. Those chunks are then stored in a `ron` file. This means being able to identify text chunks by their byte indicies in the target directory.

#### Short - Medium - Long term goals

1. Short Term: Create a working parser that identifies and extracts
   semantically relevent chunks of code like functions, structs, etc. Test with
print statements. and other basic approachs to validating program works as
intended. Will be used to make a vector embedding database by another project,
which is beyond the scope of this `parser` project.
2. Medium Term: Extract relationships and more fine-grained details. Does not
   need to be comprehensive, but should provide the data required for basic
prototyping of a heterogenous graph structure. Start performing basic data
validation of extracted data.
3. Long Term: Extract all semantically relevant information from the code that
   could be used to create a hybrid vector embedding and heterogenenous graph
database. Set up infrastructure for rigorous testing and data validation on
extracted data.

### Goals

#### Goal 1. Make better feedback mechanism

- [x] Started?
- [x] Finished?

Currently we have a simple way to display output in `main.rs`, but this should
split off into its own file in a `untils` called `print_extracted_stats.rs`

- [x] `print_extracted_stats` Split printing functions displaying numbers of
extracted items into a separate function.
- [x] Move `print_extracted_stats.rs` into the `utils` folder.
- [x] Test output and fix if needed.
- [x] Refactor for improvements
- [x] Test output again and fix if needed.

#### Goal 2. Expand the semantic chunks that can be identified and extracted

  [x] Started?
  [ ] Finished?

The project has a working approach to extracting some kinds of data using the
`tree_sitter` crate of treesitter rust bindings. We need to expand which AST
nodes the project is capable of extracting, and make sure our project's data
structures are capable of storing them.

Add node detection and extraction for the following: \

##### 1. Dependencies

- [x] `use` dependency
- [x] `mod` inclusion (in main)
  - [x] Implement extraction
  - [x] Implement saving
  - [x] Update printed table

##### 2. Enums

- [x] no fields
- [x] regular fields
- [x] `i32` and such inside
- [x] User-made structs inside
  - [x] Implement extraction
  - [x] Implement saving
  - [x] Update printed table
  - [x] Fix enum variant extraction

##### 3. Macros

- [x] Macros
  - [x] Implement extraction
  - [x] Implement saving
  - [x] Update printed table

##### 4. functions

- [x] `&self` function
- [x] non-`&self` function
  - [x] Implement extraction
  - [x] Implement saving
  - [x] Update printed table

##### 5. Add print `PrintBlock` trait and impls

- [x] Started?

Note: This will likely be helpful for later saving the data into the vector database.

- [x] Add `triat` named `PrintBlock`, which should have the following method:
  - relevant files:
    - `extract.rs` (read only)
    - `print_blocks.rs`: This is where the trait and implementations should go
  - [x] 1. function `print_block(&self)` that uses the `start_position` and
  `end_position` of our extracted info structs (e.g. `FunctionInfo`), along
  with their `file_path` field to print those bytes as a `&str` or `Sring` to
  the terminal.
- [x] 2. Implement `PrintBlock` for some of the info structs
  - relevant files:
    - `extract.rs`
    - `print_blocks.rs`: This is where the trait and implementations should go
  - Does not make sense at this point add this trait for all info structs. Just
  focus on the higher level ones.
  - [x] Finished?
  - [x] 1. Implement `PrintBlock` for the following info structs:
    - [x] `EnumInfo`
    - [x] `ModInfo`
    - [x] `MacroInfo`
    - [x] `StructInfo`
    - [x] `ImplInfo`
    - [x] `UseDependencyInfo`
    - [x] `TypeAliasInfo`
    - [x] `FunctionInfo`

##### 6. Verify `extract` Functionality for position (in bytes)

- [x] Test that calling `print_block` on the info structs works as intended.
  - relevant files:
    - `extract.rs` (read only)
    - `main.rs`
  - [x] 1. Loop through `extracted_data` in `main.rs`, printing one of each using the new method:
    - [x] `EnumInfo`
    - [x] `ModInfo`
    - [x] `MacroInfo`
    - [x] `StructInfo`
    - [x] `ImplInfo`
    - [x] `UseDependencyInfo`
    - [x] `TypeAliasInfo`
    - [x] `FunctionInfo`
  - [x] 2. Fix problems (if applicable)
  - [ ] 3. Loop through `extracted_data` in `main.rs`, and print all extracted info from the relevant structs:
    - This time we will not print them all together, but one at a time, then
    perform a brief visual inspection that it seems to be working.
    - [ ] `EnumInfo`
    - [ ] `ModInfo`
    - [ ] `MacroInfo`
    - [ ] `StructInfo`
    - [ ] `ImplInfo`
    - [ ] `UseDependencyInfo`
    - [ ] `TypeAliasInfo`
    - [ ] `FunctionInfo`
  - [ ] 4. Fix problems (if applicable)

### Someday maybe

#### Refine Parser for Heterogeneous Graphs

#### On using the `syn` crate for data extraction

##### Learn about Static Analysis

Crate a `bookmd` (location TBD) to learn more about extracting and representing
rust code in a meaningful way.

Learn about internal representation of rust tools and find information related
to my use case. Some promising avenues for learning more are:

- Following acronyms
  - HIR: High-Level Intermediate Representation
    - [ ] Rust Compiler Development Guide on [HIR]( https://rustc-dev-guide.rust-lang.org/hir.html?highlight=HIR#the-hir )
  - MIR: Mid-level Intermediate Representation
    - [ ] Rust Compiler Development Guide on [MIR]( https://rustc-dev-guide.rust-lang.org/mir/index.html#the-mir-mid-level-ir )
- MIRAI: Project for static analysis of Rust
  - [ ] [MIRAI project github](https://github.com/endorlabs/MIRAI?tab=readme-ov-file)
  - [ ] [MIRAI further reading](https://github.com/endorlabs/MIRAI/blob/main/documentation/FurtherReading.md)
- Evaluate list of projects from this awesome-list with a section on rust static analysis
  - [ ] Static analysis [awesome-list](https://github.com/analysis-tools-dev/static-analysis?tab=readme-ov-file#rust)

##### Identify and Decide on tooling for extracting info and relations

Add to book created in "Learn about Static Analysis" with following info.

Summarize, compare and contrast crates I could use to more expand the current
data structures I am using for extracted information. I would like very
fine-grained access to this information, and want it to be semantically
relevant. This is towards the long term goal of using a heterogeneous graph
database with the RAG.

Explore the following crates and learn what I need for extracting semantic relationships. Write notes in the `bookmd` just mentioned.

- [ ] `syn`
  - general summary
  - capabilities
  - relevant setup
- note: more tools here

#### Misc

- [ ] Add a way to just get the function signature of a function.
  - Should be added as a new field on the `FunctionInfo` struct.

It might be good to implement a feature flag to collect node kinds during
traversal. This would allow us to print out the unique node kinds found in a
directory, which could be useful for debugging and understanding the structure
of the code.

- Regarding this feature, there is a function in `traverse.rs` that I have used
`#ignore(dead_code)` on, which we can revisit later.

### Message to Future Self (For AI, Needs Update)

- **Project:** parser (part of rag\_workspace)
- **Goal:** Extract semantic chunks from Rust code for a hybrid graph/vector database to enhance LLM code generation and refactoring.
- **Current Task:** Implement `PrintBlock` trait and verify data extraction accuracy by byte position.
- **Progress:**
  - Implemented extraction and saving for use dependencies, mod inclusions, enums, macros, and function parameters.
  - Improved feedback mechanism with `print_extracted_stats.rs` and table formatting.
  - Refactored `StructInfoExtractor` and `FunctionInfoExtractor` to be more modular.
  - Implemented extraction of `&self` and non-`&self` functions.
- **Next Steps:**
    1. Verify the accuracy of the extracted data's byte positions by:
        - Looping through `extracted_data` in `main.rs`, printing one of each type of info struct using the new `print_block` method.
          - [ ] `EnumInfo`
          - [ ] `ModInfo`
        - Fixing any problems identified.
        - Looping through `extracted_data` in `main.rs` again, printing all extracted info from each struct type one at a time, and visually inspecting the output.
- **Medium-Term Goals:**
  - Extract relationships and more fine-grained details from the code.
  - Perform basic data validation of extracted data.
- **Long-Term Goals:**
  - Extract all semantically relevant information from the code for a hybrid vector embedding and heterogeneous graph database.
  - Set up infrastructure for rigorous testing and data validation.
- **Someday Maybe:**
  - Refine the parser for heterogeneous graphs.
  - Investigate and potentially use the `syn` crate for more advanced data extraction.
  - Add a feature to extract just the function signature.

#### LOUD WARNING FOR AI

**MAKE SURE THE `print_block` TRAIT FUNCTION TAKES `extracted_data: &ExtractedData` AS AN ARGUMENT, AND NOT `extracted_ &ExtractedData`!!!**

**NEVER EVER WRITE THIS `extracted_ &ExtractedData` !!!!!**

That warning is just an example of a mistake you (the ai) frequently make in rust syntax.
