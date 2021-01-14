# stripmargin

A little library that enables you to write multiline strings *à la* Scala.

## Usage

```rust
use stripmargin::StripMargin;

// Use '|' to set left margin, and call `strip_margin`, it's that simple!
assert_eq!(
    r#"
    |Hello,
    |  world!
    |"#
    .strip_margin(),
    "\nHello,\n  world!\n",
);

// Or you can use some other marker of your preference...
assert_eq!(
    r#"
    *Hello,
    *  world!
    *"#
    .strip_margin_with('*'),
    "\nHello,\n  world!\n",
);
```
