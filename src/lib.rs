//! A simple library that allows you to write multiline string in a more elegant way.

// https://github.com/scala/scala/blob/39148e4ec34a5c53443dd1b25ceec2308cd097fe/src/library/scala/collection/StringOps.scala#L739-L763
pub trait StripMargin {
    /// For every line in this string, strip a leading prefix consisting of blanks or control characters,
    /// followed by `margin_char` from the line.
    ///
    /// # Examples
    /// ```rust
    /// use stripmargin::StripMargin;
    /// assert_eq!(
    ///     r#"Hello,
    ///       @  world!
    ///       @"#
    ///     .strip_margin_with('@'),
    ///     "Hello,\n  world!\n",
    /// );
    /// ```
    ///
    /// Please note that a non-whitespace margin won't get stripped:
    /// ```rust
    /// use stripmargin::StripMargin;
    /// assert_eq!(
    ///     "Hello * world!".strip_margin_with('*'),
    ///     "Hello * world!",
    /// );
    /// ```
    fn strip_margin_with(&self, margin_char: char) -> String;

    /// For every line in this string, strip a leading prefix consisting of blanks or control characters,
    /// followed by `'|'` from the line.
    ///
    /// # Examples
    /// ```rust
    /// use stripmargin::StripMargin;
    /// assert_eq!(
    ///     r#"Hello,
    ///       |  world!
    ///       |"#
    ///     .strip_margin(),
    ///     "Hello,\n  world!\n",
    /// );
    /// ```
    ///
    /// Please note that a non-whitespace margin won't get stripped:
    /// ```rust
    /// use stripmargin::StripMargin;
    /// assert_eq!(
    ///     "Hello | world!".strip_margin(),
    ///     "Hello | world!",
    /// );
    /// ```
    fn strip_margin(&self) -> String {
        self.strip_margin_with('|')
    }
}

impl<S: AsRef<str>> StripMargin for S {
    fn strip_margin_with(&self, margin_char: char) -> String {
        self.as_ref()
            .split('\n')
            .map(|line| {
                let mut chars = line.chars().skip_while(|ch| ch.is_whitespace());
                match chars.next() {
                    Some(c) if c == margin_char => chars.collect(),
                    _ => line.to_owned(),
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
