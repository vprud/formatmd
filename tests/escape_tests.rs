use formatmd::format_markdown;

// TODO: fixme
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dont_escape_hash() {
        let input = "- Recalculate secondary dependencies between rounds (#378)";
        let expected = "- Recalculate secondary dependencies between rounds (#378)";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_only_escape_first_paren_or_dot() {
        let input = "1\\) Only the first \"\\)\" of a line should be escaped in this paragraph.\n\n1\\. Only the first \"\\.\" of a line should be escaped in this paragraph.\n\nFirst \\. or \\) char should not be escaped here because this line does not look like a list.";
        let expected = "1\\) Only the first \")\" of a line should be escaped in this paragraph.\n\n1\\. Only the first \".\" of a line should be escaped in this paragraph.\n\nFirst . or ) char should not be escaped here because this line does not look like a list.";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_dont_escape_list_item_marker() {
        let input = "1.No need to escape the dot here\n1)No need to escape the closing bracket here\n1. No need to escape the dot here (there is a no-break-space, not space)\n\nThis needs escaping (end of line after the dot)\n 1.\n\nThis needs escaping (space after the closing bracket)\n 1) ";
        let expected = "1.No need to escape the dot here\n1)No need to escape the closing bracket here\n1. No need to escape the dot here (there is a no-break-space, not space)\n\nThis needs escaping (end of line after the dot)\n1\\.\n\nThis needs escaping (space after the closing bracket)\n1\\)";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_escape_line_starting_plus_minus() {
        let input = "\\+\n\n\\+No need to escape plus\n+No need to escape plus\n\n\\-\n\n\\-No need to escape dash\n-No need to escape dash";
        let expected = "\\+\n\n+No need to escape plus\n+No need to escape plus\n\n\\-\n\n-No need to escape dash\n-No need to escape dash";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_escape_exclamation_preceding_link() {
        let input = "We must escape the exclamation here \\![link](https://www.debian.org/)!!!";
        let expected = "We must escape the exclamation here \\![link](https://www.debian.org/)!!!";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_asterisk_escapes() {
        let input = "Escape*asterisk\\*\n\nDon't * escape * asterisk";
        let expected = "Escape\\*asterisk\\*\n\nDon't * escape * asterisk";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_underscore_escapes() {
        let input = "Do _escape\n\nDon't esc_ape\n\nDon't _ escape _ underscore";
        let expected = "Do \\_escape\n\nDon't esc_ape\n\nDon't _ escape _ underscore";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_dont_escape_hash_not_followed_by_space() {
        let input = "#No space after hash -> no need to escape";
        let expected = "#No space after hash -> no need to escape";
        assert_eq!(format_markdown(input), expected);
    }
}
