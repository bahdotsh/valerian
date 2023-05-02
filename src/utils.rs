pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let digits_end = s
        .char_indices()
        .find_map(|(idx, c)| if c.is_ascii_digit() {None} else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];

    ( remainder, digits )
} 

pub(crate) fn extract_ops(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {},
        _ => panic!("Bad operator"),
    }

    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn extract_one_digit(){
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn not_extracting_from_empty() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_multiple() {
        assert_eq!(extract_digits("100+12"), ("+12", "100") );
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

}
