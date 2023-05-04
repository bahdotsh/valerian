pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let char_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let chars = &s[..char_end];
    let remainder = &s[char_end..];

    (remainder, chars)
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_whitespaces(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub(crate) fn extract_ops(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("Bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
    let input_starts_with_alphanumeric = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphanumeric {
        take_while(|c| c.is_ascii_alphanumeric(), s)
    } else {
        (s, "")
    }
}

pub(crate) fn tag<'a>(starting_text: &str, s: &'a str) -> &'a str {
    if s.starts_with(starting_text) {
        &s[starting_text.len()..]
    } else {
        panic!("expected: {}", starting_text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn not_extracting_from_empty() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_multiple() {
        assert_eq!(extract_digits("100+12"), ("+12", "100"));
    }

    #[test]
    fn extract_op_single() {
        assert_eq!(extract_ops("+2"), ("2", "+"));
    }

    #[test]
    fn extract_op_multiple_sub() {
        assert_eq!(extract_ops("-10"), ("10", "-"));
    }

    #[test]
    fn extract_op_multiple_mul() {
        assert_eq!(extract_ops("*23"), ("23", "*"));
    }

    #[test]
    fn extract_op_multiple_div() {
        assert_eq!(extract_ops("/1320"), ("1320", "/"));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespaces(" 1"), ("1", " "));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foobar1()"), ("()", "foobar1"));
    }

    #[test]
    fn cannot_extract_indent_beginning_with_numbers() {
        assert_eq!(extract_ident("12312asdadad"), ("12312asdadad", ""));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("=", "=a"), "a");
    }
}
