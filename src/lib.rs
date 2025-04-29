use lazy_static::lazy_static;
use markdown::{ParseOptions, mdast::Node, message::Message, to_mdast};
use regex::Regex;

lazy_static! {
    static ref RE_NEWLINES: Regex = Regex::new(r"\r\n|\r|\n").unwrap();
}

// Построение списка событий Markdown
pub fn build_mdit(input: &str) -> Result<Node, Message> {
    // TODO: Добавить возвращение events.children()
    let tree = to_mdast(input, &ParseOptions::default());
    tree
}

// Проверка равенства двух Markdown-текстов
pub fn is_md_equal(md1: &str, md2: &str) -> bool {
    let events1 = build_mdit(md1);
    let events2 = build_mdit(md2);

    // Простое сравнение списков событий.
    events1 == events2
}

// Определение типа конца строки
pub fn detect_newline_type(md: &str, eol_setting: &str) -> &'static str {
    match eol_setting {
        "keep" => {
            if let Some(first_match) = RE_NEWLINES.find(md) {
                if first_match.as_str() == "\r\n" {
                    "\r\n"
                } else {
                    "\n"
                }
            } else {
                "\n"
            }
        }
        "crlf" => "\r\n",
        _ => "\n",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_mdit() {
        let input = "# Title\nParagraph.";
        let events = build_mdit(input).unwrap();
        assert_eq!(events.children().unwrap().len(), 2);
    }

    #[test]
    fn test_is_md_equal() {
        let md1 = "# Title\nText here.";
        let md2 = "# Title\nText here.";
        assert!(is_md_equal(md1, md2));
    }

    #[test]
    fn test_detect_newline_type() {
        let md = "Line1\r\nLine2";
        let eol = detect_newline_type(md, "keep");
        assert_eq!(eol, "\r\n");
    }
}
