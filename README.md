# stripmargin

A little Rust library that enables you to write multiline strings *à la* Scala.

## Usage

```rust
use stripmargin::StripMargin;

// Use '|' to set left margin, and then `.strip_margin()`, it's that simple!
```rust
assert_eq!(
    r#"Hello,
      |  world!
      |"#
    .strip_margin(),
    "Hello,\n  world!\n",
);
```
