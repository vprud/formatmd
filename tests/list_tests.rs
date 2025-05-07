use formatmd::format_markdown;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbering_list() {
        let input = r#"
1. a
2. b
3. c
   d
"#;

        let expected = r#"1. a
2. b
3. c
   d"#;

        let formatted = format_markdown(input);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_nested_list() {
        let input = r#"
1. a
2. b
   1. x
   2. y
      z
"#;

        let expected = r#"1. a
2. b
   1. x
   2. y
      z"#;

        let formatted = format_markdown(input);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_numbered_list() {
        let input = r#"
1. a
2. b
3. c
   d
"#;

        let expected = r#"1. a
2. b
3. c
   d"#;

        let formatted = format_markdown(input);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_mixed_lists() {
        let input = r#"
1. First
2. Second
   - Nested
   - Another
3. Third
"#;

        let expected = r#"1. First
2. Second
   - Nested
   - Another
3. Third"#;

        let formatted = format_markdown(input);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_list_whitespaces() {
        let input = "- item one\n  \n- item two\n  - sublist\n  \n  - sublist";
        let expected = "- item one\n- item two\n   - sublist\n   - sublist";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_lists_with_different_bullets() {
        let input = "- a\n- b\n* c";
        let expected = "- a\n- b\n- c";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_numbered_lists() {
        let input = "1. a\n2. b\n3. c\n   d";
        let expected = "1. a\n2. b\n3. c\n   d";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_numbered_lists_starting_number() {
        let input = "099. a\n100. b\n101. c\n     d";
        let expected = "99. a\n100. b\n101. c\n   d";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_ordered_list_marker_type() {
        let input = "1) a\n2) b\n1. x\n2. y";
        let expected = "1. a\n2. b\n1. x\n2. y";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_bullet_list_marker_type() {
        let input = "* a\n* b\n+ x\n+ y\n- c\n- d\n+ e\n+ f";
        let expected = "- a\n- b\n- x\n- y\n- c\n- d\n- e\n- f";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_empty_list_item() {
        let input = "- next item is empty\n- \n- whitespace should be stripped\n\n1. next item is empty\n1. \n1. whitespace should be stripped";
        let expected = "- next item is empty\n-\n- whitespace should be stripped\n1. next item is empty\n2.\n3. whitespace should be stripped";
        assert_eq!(format_markdown(input), expected);
    }
}
