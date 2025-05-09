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
    let mut in_code_block = false;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
            }
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
            Event::End(TagEnd::Item) => {
                // Не добавляем ничего для закрывающего тега элемента списка
            }
            Event::Text(text) => {
                let processed_text = if in_code_block {
                    text.to_string()
                } else {
                    process_escapes(&text)
                };
                output.push_str(&processed_text);
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
                if let Some(text) = event_to_text(&event) {
                    output.push_str(&text);
                }
            }
        }
    }

    post_process_markdown(&output)
}

/// Обрабатывает экранирование специальных символов
fn process_escapes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    let mut in_link = false;
    let mut in_image = false;

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                if let Some(next) = chars.peek() {
                    if *next == ']' {
                        result.push('[');
                        continue;
                    }
                }
                result.push(c);
            }
            '!' => {
                if let Some(next) = chars.peek() {
                    if *next == '[' {
                        in_image = true;
                        result.push('!');
                        continue;
                    }
                }
                result.push(c);
            }
            ']' => {
                if in_link || in_image {
                    in_link = false;
                    in_image = false;
                }
                result.push(c);
            }
            '(' => {
                if in_image {
                    in_link = true;
                }
                result.push(c);
            }
            ')' => {
                if in_link {
                    in_link = false;
                }
                result.push(c);
            }
            '\\' => {
                // Пропускаем обработку экранированных символов
                if let Some(next) = chars.next() {
                    result.push(next);
                }
            }
            '*' | '_' | '`' | '~' | '+' | '-' | '=' | '!' | '<' | '>' | '|' => {
                // Экранируем только если символ может быть частью синтаксиса
                result.push('\\');
                result.push(c);
            }
            '#' => {
                // Не экранируем #, так как он может быть частью текста (например, в issue номерах)
                result.push(c);
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
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
    while output.contains("\n\n\n") {
        output = output.replace("\n\n\n", "\n\n");
    }

    // Удаление лишних пробелов в начале документа
    output.trim_start().to_string()
}