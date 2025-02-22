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
     - Use `:TSInstallInfo` for a list of installed and not installed languages.
     - Check `:messages` or `:Noice` for the list.

  2. Install language (if necessary):
     - Use the `:TSInstall <language>` command to install the parser for your language.
     - You are now ready to proceed!
     More info on [TreeSitter](https://github.com/nvim-treesitter/nvim-treesitter)

</details>

Use the `:InspectTree` command and poke around. With the cursor over a function
name you should see the corresponding identifier in the TreeSitter
representation in the right panel.

<details><summary></summary>Example for simple bevy `struct`
Here is is a bevy `struct` and the corresponding treesitter representation:

```rust
/// A [`Handle`] to the [`AnimationGraph`] to be used by the [`AnimationPlayer`](crate::AnimationPlayer) on the same entity.
#[derive(Component, Clone, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, From)]
#[reflect(Component, Default)]
pub struct AnimationGraphHandle(pub Handle<AnimationGraph>);
```

  (line_comment ; [131, 0] - [132, 0]
    outer: (outer_doc_comment_marker) ; [131, 2] - [131, 3]
    doc: (doc_comment)) ; [131, 3] - [132, 0]
  (attribute_item ; [132, 0] - [132, 90]
    (attribute ; [132, 2] - [132, 89]
      (identifier) ; [132, 2] - [132, 8]
      arguments: (token_tree ; [132, 8] - [132, 89]
        (identifier) ; [132, 9] - [132, 18]
        (identifier) ; [132, 20] - [132, 25]
        (identifier) ; [132, 27] - [132, 32]
        (identifier) ; [132, 34] - [132, 41]
        (identifier) ; [132, 43] - [132, 48]
        (identifier) ; [132, 50] - [132, 58]
        (identifier) ; [132, 60] - [132, 67]
        (identifier) ; [132, 69] - [132, 78]
        (identifier) ; [132, 80] - [132, 82]
        (identifier)))) ; [132, 84] - [132, 88]
  (attribute_item ; [133, 0] - [133, 30]
    (attribute ; [133, 2] - [133, 29]
      (identifier) ; [133, 2] - [133, 9]
      arguments: (token_tree ; [133, 9] - [133, 29]
        (identifier) ; [133, 10] - [133, 19]
        (identifier)))) ; [133, 21] - [133, 28]
  (struct_item ; [134, 0] - [134, 60]
    (visibility_modifier) ; [134, 0] - [134, 3]
    name: (type_identifier) ; [134, 11] - [134, 31]
    body: (ordered_field_declaration_list ; [134, 31] - [134, 59]
      (visibility_modifier) ; [134, 32] - [134, 35]
      type: (generic_type ; [134, 36] - [134, 58]
        type: (type_identifier) ; [134, 36] - [134, 42]
        type_arguments: (type_arguments ; [134, 42] - [134, 58]
          (type_identifier))))) ; [134, 43] - [134, 57]
</details>

<details>
    <summary>Breakdown of example:</summary>
Note: The following breakdown was provided by Google Gemini:

Here's a breakdown of the important node types and their roles:

- **`(struct_item ...)`**: This is the top-level node representing the entire `struct` definition.  It spans from the beginning of the `pub` keyword to the closing parenthesis of the struct body.

- **`(visibility_modifier)`**: This node represents the `pub` keyword, indicating public visibility. If this node is present as a child of `struct_item`, the struct is public. If it's absent, the struct has default (private) visibility.

- **`name: (type_identifier)`**:
  - `name:` indicates this child node is associated with the "name" of the struct.
  - `(type_identifier)` is the node type for identifiers used as type names. In this case, it represents `AnimationGraphHandle`, which is the name of the struct.

- **`body: (ordered_field_declaration_list ...)`**:
  - `body:` indicates this child node is associated with the struct's body (the fields).
  - `(ordered_field_declaration_list)` represents a list of field declarations within the struct.  Even for tuple structs (like this one), it's still represented as a field list, even if it's ordered and doesn't have named fields in the traditional sense.

- **Inside `ordered_field_declaration_list`**:
  - **(visibility_modifier)**:  Visibility of the field (again, `pub` in this case).
  - **`type: (generic_type ...)`**:  Specifies the type of the field.
    - **(generic_type ...)**: Indicates a generic type is being used (like `Handle<...>`).
      - **`type: (type_identifier)`**: The base type of the generic type, which is `Handle`.
      - **`type_arguments: (type_arguments ...)`**:  The type arguments within the angle brackets `<...>`.
        - **(type_identifier)**: The type argument, which is `AnimationGraph`.

### Key Node Types for Extraction

- **`struct_item`**: To identify struct definitions.
- **`visibility_modifier`**: To check for `pub` visibility.
- **`type_identifier` (within `name:` of `struct_item`)**: To get the struct name.

</details>

<a id="areas-for-improvement"></a>

## Areas for improvement

- Include file meta-data gathered while walking the target directory.
