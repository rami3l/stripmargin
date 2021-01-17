# stripmargin

![Downloads](https://img.shields.io/crates/d/stripmargin)
![License](https://img.shields.io/crates/l/stripmargin)
[![crates.io](https://img.shields.io/crates/v/stripmargin?logo=rust)](https://crates.io/crates/stripmargin)
[![docs.rs](https://docs.rs/stripmargin/badge.svg)](https://docs.rs/stripmargin)

A little Rust library that enables you to write multiline strings *à la* Scala.

## Usage

```rust
use stripmargin::StripMargin;

// Use '|' to set left margin, 
// and then `.strip_margin()` :)
```rust
assert_eq!(
    r#"Hello,
      |  world!
      |"#
    .strip_margin(),
    "Hello,\n  world!\n",
);
```
