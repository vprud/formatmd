
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
1. b
   1. x
   1. y
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
    fn test_plain_text() {
        let input = "Простой текст без разметки";
        let formatted = format_markdown(input);
        assert_eq!(formatted, input);
    }
}