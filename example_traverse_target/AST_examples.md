## Structs

### Example 1

```rust
/// Independent [`Schedule`] for stepping systems.
///
/// The stepping systems must run in their own schedule to be able to inspect
/// all the other schedules in the [`App`].  This is because the currently
/// executing schedule is removed from the [`Schedules`] resource while it is
/// being run.
#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
struct DebugSchedule;

/// Plugin to add a stepping UI to an example
#[derive(Default)]
pub struct SteppingPlugin {
    schedule_labels: Vec<InternedScheduleLabel>,
    top: Val,
    left: Val,
}
```

Above block corresponds to:

```query

  (line_comment ; [2, 0] - [3, 0]
    outer: (outer_doc_comment_marker) ; [2, 2] - [2, 3]
    doc: (doc_comment)) ; [2, 3] - [3, 0]
  (line_comment ; [3, 0] - [4, 0]
    outer: (outer_doc_comment_marker) ; [3, 2] - [3, 3]
    doc: (doc_comment)) ; [3, 3] - [4, 0]
  (line_comment ; [4, 0] - [5, 0]
    outer: (outer_doc_comment_marker) ; [4, 2] - [4, 3]
    doc: (doc_comment)) ; [4, 3] - [5, 0]
  (line_comment ; [5, 0] - [6, 0]
    outer: (outer_doc_comment_marker) ; [5, 2] - [5, 3]
    doc: (doc_comment)) ; [5, 3] - [6, 0]
  (line_comment ; [6, 0] - [7, 0]
    outer: (outer_doc_comment_marker) ; [6, 2] - [6, 3]
    doc: (doc_comment)) ; [6, 3] - [7, 0]
  (line_comment ; [7, 0] - [8, 0]
    outer: (outer_doc_comment_marker) ; [7, 2] - [7, 3]
    doc: (doc_comment)) ; [7, 3] - [8, 0]
  (attribute_item ; [8, 0] - [8, 59]
    (attribute ; [8, 2] - [8, 58]
      (identifier) ; [8, 2] - [8, 8]
      arguments: (token_tree ; [8, 8] - [8, 58]
        (identifier) ; [8, 9] - [8, 14]
        (identifier) ; [8, 16] - [8, 20]
        (identifier) ; [8, 22] - [8, 31]
        (identifier) ; [8, 33] - [8, 35]
        (identifier) ; [8, 37] - [8, 42]
        (identifier)))) ; [8, 44] - [8, 57]
  (struct_item ; [9, 0] - [9, 21]
    name: (type_identifier)) ; [9, 7] - [9, 20]
  (line_comment ; [11, 0] - [12, 0]
    outer: (outer_doc_comment_marker) ; [11, 2] - [11, 3]
    doc: (doc_comment)) ; [11, 3] - [12, 0]
  (attribute_item ; [12, 0] - [12, 18]
    (attribute ; [12, 2] - [12, 17]
      (identifier) ; [12, 2] - [12, 8]
      arguments: (token_tree ; [12, 8] - [12, 17]
        (identifier)))) ; [12, 9] - [12, 16]
  (struct_item ; [13, 0] - [17, 1]
    (visibility_modifier) ; [13, 0] - [13, 3]
    name: (type_identifier) ; [13, 11] - [13, 25]
    body: (field_declaration_list ; [13, 26] - [17, 1]
      (field_declaration ; [14, 4] - [14, 47]
        name: (field_identifier) ; [14, 4] - [14, 19]
        type: (generic_type ; [14, 21] - [14, 47]
          type: (type_identifier) ; [14, 21] - [14, 24]
          type_arguments: (type_arguments ; [14, 24] - [14, 47]
            (type_identifier)))) ; [14, 25] - [14, 46]
      (field_declaration ; [15, 4] - [15, 12]
        name: (field_identifier) ; [15, 4] - [15, 7]
        type: (type_identifier)) ; [15, 9] - [15, 12]
      (field_declaration ; [16, 4] - [16, 13]
        name: (field_identifier) ; [16, 4] - [16, 8]
        type: (type_identifier)))) ; [16, 10] - [16, 13]
```

## Macros

### Example 1

```rust
// function starts above
        if cfg!(not(feature = "bevy_debug_stepping")) {
            return;
        }
// function ends below
```

corresponds to:

```query
          (expression_statement ; [35, 8] - [37, 9]
            (if_expression ; [35, 8] - [37, 9]
              condition: (macro_invocation ; [35, 11] - [35, 53]
                macro: (identifier) ; [35, 11] - [35, 14]
                (token_tree ; [35, 15] - [35, 53]
                  (identifier) ; [35, 16] - [35, 19]
                  (token_tree ; [35, 19] - [35, 52]
                    (identifier) ; [35, 20] - [35, 27]
                    (string_literal ; [35, 30] - [35, 51]
                      (string_content))))) ; [35, 31] - [35, 50]
              consequence: (block ; [35, 54] - [37, 9]
```

## Dependencies

### Example 1

```rust
use bevy::app::MainScheduleOrder;
use bevy::{ecs::schedule::*, prelude::*};
```

Corresponds to

```query
  (use_declaration ; [0, 0] - [0, 33]
    argument: (scoped_identifier ; [0, 4] - [0, 32]
      path: (scoped_identifier ; [0, 4] - [0, 13]
        path: (identifier) ; [0, 4] - [0, 8]
        name: (identifier)) ; [0, 10] - [0, 13]
      name: (identifier))) ; [0, 15] - [0, 32]
  (use_declaration ; [1, 0] - [1, 41]
    argument: (scoped_use_list ; [1, 4] - [1, 40]
      path: (identifier) ; [1, 4] - [1, 8]
      list: (use_list ; [1, 10] - [1, 40]
        (use_wildcard ; [1, 11] - [1, 27]
          (scoped_identifier ; [1, 11] - [1, 24]
            path: (identifier) ; [1, 11] - [1, 14]
            name: (identifier))) ; [1, 16] - [1, 24]
        (use_wildcard ; [1, 29] - [1, 39]
          (identifier))))) ; [1, 29] - [1, 36]
```

For more context:

```rust
//! A simplified implementation of the classic game "Breakout".
//!
//! Demonstrates Bevy's stepping capabilities if compiled with the `bevy_debug_stepping` feature.

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

mod stepping;
```

Corresponds to:

```query
  (line_comment ; [0, 0] - [1, 0]
    inner: (inner_doc_comment_marker) ; [0, 2] - [0, 3]
    doc: (doc_comment)) ; [0, 3] - [1, 0]
  (line_comment ; [1, 0] - [2, 0]
    inner: (inner_doc_comment_marker) ; [1, 2] - [1, 3]
    doc: (doc_comment)) ; [1, 3] - [2, 0]
  (line_comment ; [2, 0] - [3, 0]
    inner: (inner_doc_comment_marker) ; [2, 2] - [2, 3]
    doc: (doc_comment)) ; [2, 3] - [3, 0]
  (use_declaration ; [4, 0] - [7, 2]
    argument: (scoped_use_list ; [4, 4] - [7, 1]
      path: (identifier) ; [4, 4] - [4, 8]
      list: (use_list ; [4, 10] - [7, 1]
        (scoped_use_list ; [5, 4] - [5, 78]
          path: (scoped_identifier ; [5, 4] - [5, 18]
            path: (identifier) ; [5, 4] - [5, 8]
            name: (identifier)) ; [5, 10] - [5, 18]
          list: (use_list ; [5, 20] - [5, 78]
            (identifier) ; [5, 21] - [5, 27]
            (identifier) ; [5, 29] - [5, 43]
            (identifier) ; [5, 45] - [5, 59]
            (identifier))) ; [5, 61] - [5, 77]
        (use_wildcard ; [6, 4] - [6, 14]
          (identifier))))) ; [6, 4] - [6, 11]
  (mod_item ; [9, 0] - [9, 13]
    name: (identifier)) ; [9, 4] - [9, 12]
```

## Implementations (`impl`)

### Example 1

```rust
impl WallLocation {
    /// Location of the *center* of the wall, used in `transform.translation()`
    fn position(&self) -> Vec2 {
        // following line for testing type alias parser detection
        let _point: Point = (1, 2);
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    /// (x, y) dimensions of the wall, used in `transform.scale()`
    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
```

Corresponds to

```query
  (impl_item ; [120, 0] - [150, 1]
    type: (type_identifier) ; [120, 5] - [120, 17]
    body: (declaration_list ; [120, 18] - [150, 1]
      (line_comment ; [121, 4] - [122, 0]
        outer: (outer_doc_comment_marker) ; [121, 6] - [121, 7]
        doc: (doc_comment)) ; [121, 7] - [122, 0]
      (function_item ; [122, 4] - [131, 5]
        name: (identifier) ; [122, 7] - [122, 15]
        parameters: (parameters ; [122, 15] - [122, 22]
          (self_parameter ; [122, 16] - [122, 21]
            (self))) ; [122, 17] - [122, 21]
        return_type: (type_identifier) ; [122, 26] - [122, 30]
        body: (block ; [122, 31] - [131, 5]
          (line_comment) ; [123, 8] - [123, 65]
          (let_declaration ; [124, 8] - [124, 35]
            pattern: (identifier) ; [124, 12] - [124, 18]
            type: (type_identifier) ; [124, 20] - [124, 25]
            value: (tuple_expression ; [124, 28] - [124, 34]
              (integer_literal) ; [124, 29] - [124, 30]
              (integer_literal))) ; [124, 32] - [124, 33]
          (expression_statement ; [125, 8] - [130, 9]
            (match_expression ; [125, 8] - [130, 9]
              value: (self) ; [125, 14] - [125, 18]
              body: (match_block ; [125, 19] - [130, 9]
                (match_arm ; [126, 12] - [126, 59]
                  pattern: (match_pattern ; [126, 12] - [126, 30]
                    (scoped_identifier ; [126, 12] - [126, 30]
                      path: (identifier) ; [126, 12] - [126, 24]
                      name: (identifier))) ; [126, 26] - [126, 30]
                  value: (call_expression ; [126, 34] - [126, 58]
                    function: (scoped_identifier ; [126, 34] - [126, 43]
                      path: (identifier) ; [126, 34] - [126, 38]
                      name: (identifier)) ; [126, 40] - [126, 43]
                    arguments: (arguments ; [126, 43] - [126, 58]
                      (identifier) ; [126, 44] - [126, 53]
                      (float_literal)))) ; [126, 55] - [126, 57]
                (match_arm ; [127, 12] - [127, 61]
                  pattern: (match_pattern ; [127, 12] - [127, 31]
                    (scoped_identifier ; [127, 12] - [127, 31]
                      path: (identifier) ; [127, 12] - [127, 24]
                      name: (identifier))) ; [127, 26] - [127, 31]
                  value: (call_expression ; [127, 35] - [127, 60]
                    function: (scoped_identifier ; [127, 35] - [127, 44]
                      path: (identifier) ; [127, 35] - [127, 39]
                      name: (identifier)) ; [127, 41] - [127, 44]
                    arguments: (arguments ; [127, 44] - [127, 60]
                      (identifier) ; [127, 45] - [127, 55]
                      (float_literal)))) ; [127, 57] - [127, 59]
                (match_arm ; [128, 12] - [128, 63]
                  pattern: (match_pattern ; [128, 12] - [128, 32]
                    (scoped_identifier ; [128, 12] - [128, 32]
                      path: (identifier) ; [128, 12] - [128, 24]
                      name: (identifier))) ; [128, 26] - [128, 32]
                  value: (call_expression ; [128, 36] - [128, 62]
                    function: (scoped_identifier ; [128, 36] - [128, 45]
                      path: (identifier) ; [128, 36] - [128, 40]
                      name: (identifier)) ; [128, 42] - [128, 45]
                    arguments: (arguments ; [128, 45] - [128, 62]
                      (float_literal) ; [128, 46] - [128, 48]
                      (identifier)))) ; [128, 50] - [128, 61]
                (match_arm ; [129, 12] - [129, 57]
                  pattern: (match_pattern ; [129, 12] - [129, 29]
                    (scoped_identifier ; [129, 12] - [129, 29]
                      path: (identifier) ; [129, 12] - [129, 24]
                      name: (identifier))) ; [129, 26] - [129, 29]
                  value: (call_expression ; [129, 33] - [129, 56]
                    function: (scoped_identifier ; [129, 33] - [129, 42]
                      path: (identifier) ; [129, 33] - [129, 37]
                      name: (identifier)) ; [129, 39] - [129, 42]
                    arguments: (arguments ; [129, 42] - [129, 56]
                      (float_literal) ; [129, 43] - [129, 45]
                      (identifier))))))))) ; [129, 47] - [129, 55]
      (line_comment ; [133, 4] - [134, 0]
        outer: (outer_doc_comment_marker) ; [133, 6] - [133, 7]
        doc: (doc_comment)) ; [133, 7] - [134, 0]
      (function_item ; [134, 4] - [149, 5]
        name: (identifier) ; [134, 7] - [134, 11]
        parameters: (parameters ; [134, 11] - [134, 18]
          (self_parameter ; [134, 12] - [134, 17]
            (self))) ; [134, 13] - [134, 17]
        return_type: (type_identifier) ; [134, 22] - [134, 26]
        body: (block ; [134, 27] - [149, 5]
          (let_declaration ; [135, 8] - [135, 50]
            pattern: (identifier) ; [135, 12] - [135, 24]
            value: (binary_expression ; [135, 27] - [135, 49]
              left: (identifier) ; [135, 27] - [135, 35]
              right: (identifier))) ; [135, 38] - [135, 49]
          (let_declaration ; [136, 8] - [136, 49]
            pattern: (identifier) ; [136, 12] - [136, 23]
            value: (binary_expression ; [136, 26] - [136, 48]
              left: (identifier) ; [136, 26] - [136, 36]
              right: (identifier))) ; [136, 39] - [136, 48]
          (line_comment) ; [137, 8] - [137, 55]
          (expression_statement ; [138, 8] - [138, 36]
            (macro_invocation ; [138, 8] - [138, 35]
              macro: (identifier) ; [138, 8] - [138, 14]
              (token_tree ; [138, 15] - [138, 35]
                (source_file ; [138, 15] - [138, 35]
                  (expression_statement ; [138, 15] - [138, 35]
                    (parenthesized_expression ; [138, 15] - [138, 35]
                      (binary_expression ; [138, 16] - [138, 34]
                        left: (identifier) ; [138, 16] - [138, 28]
                        right: (float_literal))))) ; [138, 31] - [138, 34]
                (identifier) ; [138, 16] - [138, 28]
                (float_literal)))) ; [138, 31] - [138, 34]
          (expression_statement ; [139, 8] - [139, 35]
            (macro_invocation ; [139, 8] - [139, 34]
              macro: (identifier) ; [139, 8] - [139, 14]
              (token_tree ; [139, 15] - [139, 34]
                (source_file ; [139, 15] - [139, 34]
                  (expression_statement ; [139, 15] - [139, 34]
                    (parenthesized_expression ; [139, 15] - [139, 34]
                      (binary_expression ; [139, 16] - [139, 33]
                        left: (identifier) ; [139, 16] - [139, 27]
                        right: (float_literal))))) ; [139, 30] - [139, 33]
                (identifier) ; [139, 16] - [139, 27]
                (float_literal)))) ; [139, 30] - [139, 33]
          (expression_statement ; [141, 8] - [148, 9]
            (match_expression ; [141, 8] - [148, 9]
              value: (self) ; [141, 14] - [141, 18]
              body: (match_block ; [141, 19] - [148, 9]
                (match_arm ; [142, 12] - [144, 13]
                  pattern: (match_pattern ; [142, 12] - [142, 52]
                    (or_pattern ; [142, 12] - [142, 52]
                      (scoped_identifier ; [142, 12] - [142, 30]
                        path: (identifier) ; [142, 12] - [142, 24]
                        name: (identifier)) ; [142, 26] - [142, 30]
                      (scoped_identifier ; [142, 33] - [142, 52]
                        path: (identifier) ; [142, 33] - [142, 45]
                        name: (identifier)))) ; [142, 47] - [142, 52]
                  value: (block ; [142, 56] - [144, 13]
                    (call_expression ; [143, 16] - [143, 72]
                      function: (scoped_identifier ; [143, 16] - [143, 25]
                        path: (identifier) ; [143, 16] - [143, 20]
                        name: (identifier)) ; [143, 22] - [143, 25]
                      arguments: (arguments ; [143, 25] - [143, 72]
                        (identifier) ; [143, 26] - [143, 40]
                        (binary_expression ; [143, 42] - [143, 71]
                          left: (identifier) ; [143, 42] - [143, 54]
                          right: (identifier)))))) ; [143, 57] - [143, 71]
                (match_arm ; [145, 12] - [147, 13]
                  pattern: (match_pattern ; [145, 12] - [145, 52]
                    (or_pattern ; [145, 12] - [145, 52]
                      (scoped_identifier ; [145, 12] - [145, 32]
                        path: (identifier) ; [145, 12] - [145, 24]
                        name: (identifier)) ; [145, 26] - [145, 32]
                      (scoped_identifier ; [145, 35] - [145, 52]
                        path: (identifier) ; [145, 35] - [145, 47]
                        name: (identifier)))) ; [145, 49] - [145, 52]
                  value: (block ; [145, 56] - [147, 13]
                    (call_expression ; [146, 16] - [146, 71]
                      function: (scoped_identifier ; [146, 16] - [146, 25]
                        path: (identifier) ; [146, 16] - [146, 20]
                        name: (identifier)) ; [146, 22] - [146, 25]
                      arguments: (arguments ; [146, 25] - [146, 71]
                        (binary_expression ; [146, 26] - [146, 54]
                          left: (identifier) ; [146, 26] - [146, 37]
                          right: (identifier)) ; [146, 40] - [146, 54]
                        (identifier)))))))))))) ; [146, 56] - [146, 70]
```

### Example 2

```rust
impl Wall {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    // Notice the use of Sprite and Transform alongside Wall, overwriting the default values defined for the required components
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: location.position().extend(0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
}
```

Corresponds to:

```query
  (impl_item ; [152, 0] - [172, 1]
    type: (type_identifier) ; [152, 5] - [152, 9]
    body: (declaration_list ; [152, 10] - [172, 1]
      (line_comment) ; [153, 4] - [153, 79]
      (line_comment) ; [154, 4] - [154, 85]
      (line_comment) ; [155, 4] - [155, 128]
      (function_item ; [156, 4] - [171, 5]
        name: (identifier) ; [156, 7] - [156, 10]
        parameters: (parameters ; [156, 10] - [156, 34]
          (parameter ; [156, 11] - [156, 33]
            pattern: (identifier) ; [156, 11] - [156, 19]
            type: (type_identifier))) ; [156, 21] - [156, 33]
        return_type: (tuple_type ; [156, 38] - [156, 63]
          (type_identifier) ; [156, 39] - [156, 43]
          (type_identifier) ; [156, 45] - [156, 51]
          (type_identifier)) ; [156, 53] - [156, 62]
        body: (block ; [156, 64] - [171, 5]
          (tuple_expression ; [157, 8] - [170, 9]
            (identifier) ; [158, 12] - [158, 16]
            (call_expression ; [159, 12] - [159, 53]
              function: (scoped_identifier ; [159, 12] - [159, 30]
                path: (identifier) ; [159, 12] - [159, 18]
                name: (identifier)) ; [159, 20] - [159, 30]
              arguments: (arguments ; [159, 30] - [159, 53]
                (identifier) ; [159, 31] - [159, 41]
                (scoped_identifier ; [159, 43] - [159, 52]
                  path: (identifier) ; [159, 43] - [159, 47]
                  name: (identifier)))) ; [159, 49] - [159, 52]
            (struct_expression ; [160, 12] - [169, 13]
              name: (type_identifier) ; [160, 12] - [160, 21]
              body: (field_initializer_list ; [160, 22] - [169, 13]
                (line_comment) ; [161, 16] - [161, 87]
                (line_comment) ; [162, 16] - [162, 69]
                (field_initializer ; [163, 16] - [163, 60]
                  field: (field_identifier) ; [163, 16] - [163, 27]
                  value: (call_expression ; [163, 29] - [163, 60]
                    function: (field_expression ; [163, 29] - [163, 55]
                      value: (call_expression ; [163, 29] - [163, 48]
                        function: (field_expression ; [163, 29] - [163, 46]
                          value: (identifier) ; [163, 29] - [163, 37]
                          field: (field_identifier)) ; [163, 38] - [163, 46]
                        arguments: (arguments)) ; [163, 46] - [163, 48]
                      field: (field_identifier)) ; [163, 49] - [163, 55]
                    arguments: (arguments ; [163, 55] - [163, 60]
                      (float_literal)))) ; [163, 56] - [163, 59]
                (line_comment) ; [164, 16] - [164, 64]
                (line_comment) ; [165, 16] - [165, 73]
                (line_comment) ; [166, 16] - [166, 69]
                (field_initializer ; [167, 16] - [167, 50]
                  field: (field_identifier) ; [167, 16] - [167, 21]
                  value: (call_expression ; [167, 23] - [167, 50]
                    function: (field_expression ; [167, 23] - [167, 45]
                      value: (call_expression ; [167, 23] - [167, 38]
                        function: (field_expression ; [167, 23] - [167, 36]
                          value: (identifier) ; [167, 23] - [167, 31]
                          field: (field_identifier)) ; [167, 32] - [167, 36]
                        arguments: (arguments)) ; [167, 36] - [167, 38]
                      field: (field_identifier)) ; [167, 39] - [167, 45]
                    arguments: (arguments ; [167, 45] - [167, 50]
                      (float_literal)))) ; [167, 46] - [167, 49]
                (base_field_initializer ; [168, 16] - [168, 27]
                  (call_expression ; [168, 18] - [168, 27]
                    function: (identifier) ; [168, 18] - [168, 25]
                    arguments: (arguments)))))))))) ; [168, 25] - [168, 27]
```

## Enums

### Example 1

```rust
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}
```

Corresponds to

```query
  (attribute_item ; [412, 0] - [412, 44]
    (attribute ; [412, 2] - [412, 43]
      (identifier) ; [412, 2] - [412, 8]
      arguments: (token_tree ; [412, 8] - [412, 43]
        (identifier) ; [412, 9] - [412, 14]
        (identifier) ; [412, 16] - [412, 25]
        (identifier) ; [412, 27] - [412, 29]
        (identifier) ; [412, 31] - [412, 35]
        (identifier)))) ; [412, 37] - [412, 42]
  (enum_item ; [413, 0] - [418, 1]
    name: (type_identifier) ; [413, 5] - [413, 14]
    body: (enum_variant_list ; [413, 15] - [418, 1]
      (enum_variant ; [414, 4] - [414, 8]
        name: (identifier)) ; [414, 4] - [414, 8]
      (enum_variant ; [415, 4] - [415, 9]
        name: (identifier)) ; [415, 4] - [415, 9]
      (enum_variant ; [416, 4] - [416, 7]
        name: (identifier)) ; [416, 4] - [416, 7]
      (enum_variant ; [417, 4] - [417, 10]
        name: (identifier)))) ; [417, 4] - [417, 10]

```

### Example 2

```rust
enum Example {}
```

Corresponds to:

```query
  (enum_item ; [420, 0] - [420, 15]
    name: (type_identifier) ; [420, 5] - [420, 12]
    body: (enum_variant_list)) ; [420, 13] - [420, 15]
```

### Example 3

```rust
enum NewThing {
    Some(i32),
    Another(String),
    Nothing,
}
```

Corresponds to

```query
  (enum_item ; [422, 0] - [426, 1]
    name: (type_identifier) ; [422, 5] - [422, 13]
    body: (enum_variant_list ; [422, 14] - [426, 1]
      (enum_variant ; [423, 4] - [423, 13]
        name: (identifier) ; [423, 4] - [423, 8]
        body: (ordered_field_declaration_list ; [423, 8] - [423, 13]
          type: (primitive_type))) ; [423, 9] - [423, 12]
      (enum_variant ; [424, 4] - [424, 19]
        name: (identifier) ; [424, 4] - [424, 11]
        body: (ordered_field_declaration_list ; [424, 11] - [424, 19]
          type: (type_identifier))) ; [424, 12] - [424, 18]
      (enum_variant ; [425, 4] - [425, 11]
        name: (identifier)))) ; [425, 4] - [425, 11]
```

### Example 4

```rust
enum Matrix {
    Location(Pos),
    Nowhere,
    WithoutReason(NewThing),
}
```

```query
  (enum_item ; [433, 0] - [437, 1]
    name: (type_identifier) ; [433, 5] - [433, 11]
    body: (enum_variant_list ; [433, 12] - [437, 1]
      (enum_variant ; [434, 4] - [434, 17]
        name: (identifier) ; [434, 4] - [434, 12]
        body: (ordered_field_declaration_list ; [434, 12] - [434, 17]
          type: (type_identifier))) ; [434, 13] - [434, 16]
      (enum_variant ; [435, 4] - [435, 11]
        name: (identifier)) ; [435, 4] - [435, 11]
      (enum_variant ; [436, 4] - [436, 27]
        name: (identifier) ; [436, 4] - [436, 17]
        body: (ordered_field_declaration_list ; [436, 17] - [436, 27]
          type: (type_identifier))))) ; [436, 18] - [436, 26]
```

##
