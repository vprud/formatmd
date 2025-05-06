use pulldown_cmark::{Event, Parser, Tag, TagEnd};

const INDENT: &str = "   "; // 4 пробела для отступов

/// Форматирует Markdown-текст с правильной нумерацией и отступами
pub fn format_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut output = String::new();
    let mut list_stack = Vec::new(); // Трекер вложенных списков
    
    for event in parser {
        match event {
            Event::Start(Tag::List(Some(start_num))) => {
                // Начало нумерованного списка
                list_stack.push(ListInfo {
                    is_ordered: true,
                    current_num: start_num,
                    max_num: start_num, // Будем обновлять при обработке
                    depth: list_stack.len() + 1,
                });
            }
            Event::Start(Tag::List(None)) => {
                // Начало ненумерованного списка
                list_stack.push(ListInfo {
                    is_ordered: false,
                    current_num: 0,
                    max_num: 0,
                    depth: list_stack.len() + 1,
                });
            }
            Event::End(TagEnd::List(_)) => {
                // Конец списка
                list_stack.pop();
            }
            Event::Start(Tag::Item) => {
                if let Some(current_list) = list_stack.last_mut() {
                    let indent = INDENT.repeat(current_list.depth - 1);
                    
                    if current_list.is_ordered {
                        // Определяем количество цифр для форматирования
                        let digits = count_digits(current_list.max_num);
                        let formatted_num = format!("{:0digits$}", current_list.current_num);
                        
                        output.push_str(&format!("\n{}{}. ", indent, formatted_num));
                        current_list.current_num += 1;
                    } else {
                        output.push_str(&format!("\n{}- ", indent));
                    }
                }
            }
            Event::Text(text) => {
                output.push_str(&text);
                
                // Обновляем max_num если это нумерованный список
                if let Some(current_list) = list_stack.last_mut() {
                    if current_list.is_ordered {
                        if let Some(num) = extract_number(&text) {
                            if num > current_list.max_num {
                                current_list.max_num = num;
                            }
                        }
                    }
                }
            }
            Event::SoftBreak => {
                output.push('\n');
                if let Some(current_list) = list_stack.last() {
                    output.push_str(&INDENT.repeat(current_list.depth));
                }
            }
            Event::HardBreak => {
                output.push_str("\n\n");
                if let Some(current_list) = list_stack.last() {
                    output.push_str(&INDENT.repeat(current_list.depth));
                }
            }
            _ => {}
        }
    }

    output.trim_start().to_string()
}

/// Информация о текущем списке
struct ListInfo {
    is_ordered: bool,
    current_num: u64,
    max_num: u64,
    depth: usize,
}

/// Извлекает число из текста (для определения максимального номера)
fn extract_number(s: &str) -> Option<u64> {
    let mut num_str = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            num_str.push(c);
        } else if !num_str.is_empty() {
            break;
        }
    }
    num_str.parse().ok()
}

/// Подсчитывает количество цифр в числе
fn count_digits(n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut num = n;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}