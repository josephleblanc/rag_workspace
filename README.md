# rag_workspace

Workspace for playing with RAGs

The goal is to use semantic chunking to create meaningful chunks of code from
the target directory, which can then be processed with vector embedding and
ultimately added to a vector database. This vector database will be used by the
RAG to augment the prompt sent to the LLM to enhance its ability to generate or
refactor code that uses semantic structures from the target directory.

Overall workflow:

## Step 1: Generate the tree-sitter syntax tree for the target project

1. Install the tree-sitter crate:
Follow these [tree-sitter instructions](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust).
2. Use the `walkdir` rust crate to traverse the target project directory
While an intermediate stage, there is helpful data that could be generated here. For now, we will just pass the file contents to `tree-sitter` for parsing.

Note [Areas for Improvement](#areas-for-improvement) below.

3. Pass contents to tree-sitter for parsing

### Step 2: Extract

1. Identify meaningful code structures.

<details>
    <summary>More on choosing relevant code structures</summary>
Note: The following was created with Gemini 2.0 Flash Thinking Experimental with Apps.

### Defining "Semantic Chunks" for Code RAG

For code generation and refactoring, especially in Rust, good semantic chunks to start with are likely to be definitions and potentially high-level code structures.  Here are some initial chunk types we can consider:

#### Function Definitions

These are fundamental units of code logic.  For each function, we'd want to extract:

- **Function Signature**:  The entire function definition line (e.g., `fn my_function(arg1: i32, arg2: String) -> Result<(), Error>`).
- **Function Body**: The code within the curly braces `{}`.
- **Function Name**: `my_function`.
- **Documentation/Doc Comments**:  Any `///` or `/** */` comments immediately preceding the function definition.
- **Scope/Context** (Optional for now):  For more advanced scenarios, we might want to know if it's a method of a struct/trait, or in a specific module, but let's keep it simpler initially.

#### Struct Definitions

Structs define data structures. Relevant information:

- **Struct Definition**:  `struct MyStruct { ... }`.
- **Struct Name**: `MyStruct`.
- **Fields**:  Names and types of fields within the struct.
- **Documentation/Doc Comments**:  Doc comments for the struct itself and potentially for individual fields.

#### Enum Definitions

Enums define sets of possible values. Relevant information:

- **Enum Definition**: `enum MyEnum { ... }`.
- **Enum Name**: `MyEnum`.
- **Variants**:  Names of enum variants.
- **Documentation/Doc Comments**: Doc comments for the enum and variants.

#### Trait Definitions (Rust Specific)

Traits define interfaces. Relevant information:

- **Trait Definition**: `trait MyTrait { ... }`.
- **Trait Name**: `MyTrait`.
- **Associated Items**:  Methods, types, constants defined within the trait.
- **Documentation/Doc Comments**: Doc comments for the trait.

</details>

We will start with the simple approach of chunking by function definition to develop our working prototype.

2. Find the tree-sitter syntax for the semantic structures (e.g. functions)

We have identified that we want to look at functions. Now to find the way the tree-sitter syntax for those functions. We use the built-in tree-sitter functionality in NeoVim to explore a file in our target directory.

<details>
<summary>Set up TreeSitter for language (if necessary)</summary>
1. Check whether the TreeSitter parser is installed for target language already:
   * Use `:TSInstallInfo` for a list of installed and not installed languages.
   * Check `:messages` or `:Noice` for the list.
2. Install language (if necessary):
   * Use the `:TSInstall <language>` command to install the parser for your language.
   * You are now ready to proceed!
   More info on [TreeSitter](https://github.com/nvim-treesitter/nvim-treesitter)
</details>

Use the `:InspectTree` command

<a id="areas-for-improvement"></a>

## Areas for improvement

- Include file meta-data gathered while walking the target directory.
