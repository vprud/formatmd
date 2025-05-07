use formatmd::format_markdown;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_paragraph_lines() {
        let input = "trailing whitespace \nat the end of paragraph lines \nshould be stripped                   ";
        let expected = "trailing whitespace\nat the end of paragraph lines\nshould be stripped";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_strip_quotes() {
        let input = "> Paragraph 1\n> \n> Paragraph 2";
        let expected = "> Paragraph 1\n>\n> Paragraph 2";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_no_escape_ampersand() {
        let input = "R&B, rock & roll";
        let expected = "R&B, rock & roll";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_convert_setext_to_atx_heading() {
        let input = "Top level heading\n=========\n\n2nd level heading\n---------";
        let expected = "# Top level heading\n\n## 2nd level heading";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_references() {
        let input = "[ref2]: link3 \"title\"\n\n[text](link1) [text](link2 \"title\") [ref1] [ref2] [text][ref1]\n\n![text](link1) ![text](link2 \"title\") ![ref1] ![ref2] ![text][ref1]\n\n[ref1]: link4\n[unused]: link5";
        let expected = "[text](link1) [text](link2 \"title\") [ref1] [ref2] [text][ref1]\n\n![text](link1) ![text](link2 \"title\") ![ref1] ![ref2] ![text][ref1]\n\n[ref1]: link4\n[ref2]: link3 \"title\"";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_thematic_breaks() {
        let input = "something something\n\n---\n\nsomething something";
        let expected = "something something\n\n______________________________________________________________________\n\nsomething something";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_empty_ref_link_destination() {
        let input = "[foo]: <>\n\n[foo]";
        let expected = "[foo]\n\n[foo]: <>";
        assert_eq!(format_markdown(input), expected);
    }


    #[test]
    // TODO: fixme
    fn test_keep_shortcut_reference_links() {
        let input = "![Image]\n\n[iMaGe]: train.jpg\n\n[Foo]\n\n[fOO]: /url \"title\"";
        let expected = "![Image]\n\n[Foo]\n\n[foo]: /url \"title\"\n[image]: train.jpg";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_empty_document() {
        let input = "";
        let expected = "";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_indented_raw_html_contains_markdown() {
        let input = "  <center>\n\n - list item\n\n  </center>";
        let expected = "<center>\n\n- list item\n\n</center>";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_autolink_with_percentage_encoded_space() {
        let input = "<https://mytest.com/files/word%20document.docx>";
        let expected = "<https://mytest.com/files/word%20document.docx>";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_keep_mailto_prefix() {
        let input = "<MAILTO:FOO@BAR.BAZ>>";
        let expected = "<MAILTO:FOO@BAR.BAZ>>";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_tilde_in_info_string() {
        let input = "```~/.gitconfig\n[user]\n```";
        let expected = "```~/.gitconfig\n[user]\n```";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    // TODO: fixme
    fn test_info_string_starts_with_tilde_and_contains_backtick() {
        let input = "~~~\\~`\n123\n~~~";
        let expected = "~~~\\~`\n123\n~~~";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_keep_inline_html_as_inline_and_block_html_as_block() {
        let input = "A\n    <div>\n\nA\n   <div>";
        let expected = "A\n    <div>\n\nA\n\n<div>";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_keep_html_blocks_of_type_7_unindented() {
        let input = "text\n<br/>\ntext";
        let expected = "text\n<br/>\ntext";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_sort_digit_references_numerically() {
        let input = "(References)[1] (should)[2] (sort)[10] (numerically)[word], if[🤪] they[⅕] are[𐩂] digits[!].\n\n[🤪]: .\n[2]: .\n[word]: .\n[10]: .\n[⅕]: .\n[1]: .\n[𐩂]: .\n[!]: .";
        let expected = "(References)[1] (should)[2] (sort)[10] (numerically)[word], if[🤪] they[⅕] are[𐩂] digits[!].\n\n[!]: .\n[1]: .\n[2]: .\n[10]: .\n[word]: .\n[⅕]: .\n[𐩂]: .\n[🤪]: .";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_unicode_space_after_heading() {
        let input = "# hoge\n　\n";
        let expected = "# hoge";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_square_bracket_escapes() {
        let input = "[no-escape]no [no-escape] no [\\[\\]](/url)\n\n[escape]\n\n[inline\\](/url)\n\n[link-label]\n\n[link-label\\]\n\n[link-label\\]: /url\n\n[link-label]: /url";
        let expected = "[no-escape]no [no-escape] no [[]](/url)\n\n[escape]\n\n\\[inline\\](/url)\n\n[link-label]\n\n\\[link-label\\]\n\n\\[link-label\\]: /url\n\n[link-label]: /url";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_less_than_sign_escapes() {
        let input = "< no escape < no escape, now escape <\n<\n\n<";
        let expected = "< no escape < no escape, now escape \\<\n\\<\n\n\\<";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_tabs_to_spaces() {
        let input = "# Convert\ttab\tto\tspace\tin\t\t\theadings\n\nMake a space\there.";
        let expected = "# Convert tab to space in headings\n\nMake a space here.";
        assert_eq!(format_markdown(input), expected);
    }

    #[test]
    fn test_reduce_tabs_and_spaces_to_one_space() {
        let input = "# Reduce   \t\tin\t\t\t  \theadings\n\nReduce to a space\t \there.";
        let expected = "# Reduce in headings\n\nReduce to a space here.";
        assert_eq!(format_markdown(input), expected);
    }
}
