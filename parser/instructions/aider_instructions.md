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

The intended database will be a heterogeneous graph database. This means it will
be important to identify relationships between data structures. Those
relashionships will later allow us to create edges in the graph to represent
meaningful connections, and improve the capability of the RAG and thereby the
LLM doing code generation and refactoring.

The project aims to accept a directory as input, and then parse the `.rs` files in that library for relevant chunks of data. Those chunks are then stored in a `ron` file. This means being able to identify text chunks by their byte indicies in the target directory.

### Goals

#### Goal 1. Make better feedback mechanism

- [ ] Started?
- [ ] Finished?

Currently we have a simple way to display output in `main.rs`, but this should
split off into its own file in a `untils` called `print_extracted_stats.rs`

- [x] `print_extracted_stats` Split printing functions displaying numbers of
extracted items into a separate function.
- [x] Move `print_extracted_stats.rs` into the `utils` folder.
- [ ] Test output and fix if needed.
- [ ] Refactor for improvements
- [ ] Test output again and fix if needed.
- [ ] Add improvements to printed formatting:
  - [ ] print stats in table

**1.1**

#### Goal 2. Expand the semantic chunks that can be identified and extracted

  [x] Started?
  [ ] Finished?

The project has a working approach to extracting some kinds of data using the
`tree_sitter` crate of treesitter rust bindings. We need to expand which AST
nodes the project is capable of extracting, and make sure our project's data
structures are capable of storing them.

Add node detection and extraction for the following: \

##### 1. Dependencies

- [ ] `use` dependency
- [ ] `mod` inclusion (in main)
- [ ] update `parser/src/traverse.rs` with the new extractor

##### 2. Enums

- [ ] no fields
- [ ] regular fields
- [ ] `i32` and such inside
- [ ] User-made structs inside
Incorporate the new changes
- [ ] update `parser/src/traverse.rs` with the new extractor
- [ ] update `` with the new extractor

##### 3. Macros

- [ ] Macros

##### 4. functions

- [ ] parameters
- [ ] `&self` function
- [ ] non-`&self` function

##### \<maybe more here later\>

- [ ] task
- [ ] task
