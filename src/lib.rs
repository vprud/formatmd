use pulldown_cmark::{Event, Parser, Tag, TagEnd};


/// Форматирует Markdown-текст, сохраняя структуру (включая вложенные списки)
/// и применяя заданные стили форматирования.
pub fn format_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut output = String::new();
    let mut in_list = false;
    let mut list_depth = 0;
    let mut list_number: Option<u64> = None;

    for event in parser {
        match event {
            Event::Start(Tag::List(Some(start))) => {
                in_list = true;
                list_depth += 1;
                list_number = Some(start);
            }
            Event::End(TagEnd::List(_)) => {
                list_depth -= 1;
                if list_depth == 0 {
                    in_list = false;
                }
                list_number = None
            }
            Event::Start(Tag::Item) => {
                let indentItem = indent(list_depth - 1);
                if let Some(num) = list_number {
                    output.push_str(&format!("\n{}{}. ", indentItem, num));
                    list_number = Some(num + 1); // Увеличиваем номер
                } else {
                    output.push_str(&format!("\n{}- ", indentItem));
                }
            }
            Event::Text(text) => {
                output.push_str(&text);
            }
            Event::SoftBreak => {
                output.push('\n');
                if in_list {
                    output.push_str(&indent(list_depth));
                }
            }
            Event::HardBreak => {
                output.push_str("\n\n");
                if in_list {
                    output.push_str(&indent(list_depth));
                }
            }
            _ => {}
        }
    }

    output.trim_start().to_string()
}

fn indent(depth: usize) -> String {
    "   ".repeat(depth)
}