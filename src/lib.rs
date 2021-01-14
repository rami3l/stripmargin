// https://github.com/scala/scala/blob/39148e4ec34a5c53443dd1b25ceec2308cd097fe/src/library/scala/collection/StringOps.scala#L739-L763
pub trait StripMargin {
    /// For every line in this string, strip a leading prefix consisting of blanks or control characters,
    /// followed by `margin_char` from the line.
    fn strip_margin_with(&self, margin_char: char) -> String;

    /// For every line in this string, strip a leading prefix consisting of blanks or control characters,
    /// followed by `'|'` from the line.
    fn strip_margin(&self) -> String {
        self.strip_margin_with('|')
    }
}

impl<S: AsRef<str>> StripMargin for S {
    fn strip_margin_with(&self, margin_char: char) -> String {
        self.as_ref()
            .split_inclusive('\n')
            .map(|line| {
                let mut chars = line.chars().skip_while(|ch| ch.is_whitespace());
                match chars.next() {
                    Some(c) if c == margin_char => chars.collect(),
                    _ => line.to_owned(),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(
            "".strip_margin(),
            "",
            "strip_margin should not modify an empty string"
        );
    }

    #[test]
    fn non_blank_prefix() {
        assert_eq!(
            "Hello | world!".strip_margin(),
            "Hello | world!",
            "strip_margin should not modify a non-blank prefix"
        );
    }

    #[test]
    fn blank_prefix() {
        assert_eq!(
            r#"
            |Hello,
            |  world!
            |"#
            .strip_margin(),
            "\nHello,\n  world!\n",
            "strip_margin should eliminate blank prefixes"
        );
    }

    #[test]
    fn custom_margin_char() {
        assert_eq!(
            r#"
            *Hello,
            *  world!
            *"#
            .strip_margin_with('*'),
            "\nHello,\n  world!\n",
            "strip_margin_with should handle custom margin_char"
        );
    }
}