use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

const INDENT: &str = "   "; // 4 пробела для отступов

/// Форматирует Markdown-текст с правильной структурой
pub fn format_markdown(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(input, options);
    let mut output = String::new();
    let mut list_stack = Vec::new(); // Трекер вложенных списков

    for event in parser {
        match event {
            Event::Start(Tag::List(Some(start_num))) => {
                list_stack.push(ListInfo {
                    is_ordered: true,
                    current_num: start_num,
                    depth: list_stack.len() + 1,
                });
            }
            Event::Start(Tag::List(None)) => {
                list_stack.push(ListInfo {
                    is_ordered: false,
                    current_num: 0,
                    depth: list_stack.len() + 1,
                });
            }
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
            }
            Event::Start(Tag::Item) => {
                if let Some(current_list) = list_stack.last_mut() {
                    let indent = INDENT.repeat(current_list.depth - 1);

                    if current_list.is_ordered {
                        output.push_str(&format!("\n{}{}. ", indent, current_list.current_num));
                        current_list.current_num += 1;
                    } else {
                        output.push_str(&format!("\n{}- ", indent));
                    }
                }
            }
            Event::Text(text) => {
                // Обработка текста с сохранением оригинального форматирования
                output.push_str(&text);
            }
            Event::SoftBreak => {
                output.push('\n');
                if let Some(current_list) = list_stack.last() {
                    output.push_str(&INDENT.repeat(current_list.depth));
                }
            }
            Event::HardBreak => {
                output.push_str("\n\n");
            }
            _ => {
                // Для всех других событий просто сохраняем оригинальное содержимое
                if let Some(text) = event_to_text(&event) {
                    output.push_str(&text);
                }
            }
        }
    }

    // Пост-обработка для удаления лишних пробелов и приведения к стандартному формату
    post_process_markdown(&output)
}

/// Информация о текущем списке
struct ListInfo {
    is_ordered: bool,
    current_num: u64,
    depth: usize,
}

/// Конвертирует событие в текст (упрощенная версия)
fn event_to_text(event: &Event) -> Option<String> {
    match event {
        Event::Text(text) => Some(text.to_string()),
        Event::Code(code) => Some(format!("`{}`", code)),
        Event::Html(html) => Some(html.to_string()),
        _ => None,
    }
}

/// Пост-обработка Markdown для соответствия тест-кейсам
fn post_process_markdown(input: &str) -> String {
    let mut output = input.to_string();

    // Удаление лишних пробелов в конце строк
    output = output
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");

    // Нормализация пустых строк между блоками
    output = output.replace("\n\n\n", "\n\n");

    // Удаление лишних пробелов в начале документа
    output.trim_start().to_string()
}
