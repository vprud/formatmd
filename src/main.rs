use pulldown_cmark::{Options, Parser, Tag, TagEnd};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};
use std::path::{Path};
use std::error::Error;
use std::fmt;

pub struct FormatterConfig {
    pub wrap_width: usize,
    pub max_header_level: u32,
    pub standardize_horizontal_lines: bool,
    pub normalize_spaces: bool,
    pub remove_extra_blank_lines: bool,
    pub uniform_links_style: bool,
    pub indent_size: u32,
    pub consecutive_numbering: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        FormatterConfig {
            wrap_width: 80,
            max_header_level: 6,
            standardize_horizontal_lines: true,
            normalize_spaces: true,
            remove_extra_blank_lines: true,
            uniform_links_style: true,
            indent_size: 4,
            consecutive_numbering: true,
        }
    }
}

pub struct MarkdownFormatter<'a> {
    config: &'a FormatterConfig,
}

impl<'a> MarkdownFormatter<'a> {
    pub fn new(config: &'a FormatterConfig) -> Self {
        MarkdownFormatter { config }
    }

    pub fn format_markdown(&self, input_path: PathBuf, output_path: Option<PathBuf>) -> Result<(), String> {
        let mut buffer = Vec::new();
        let mut file = File::open(input_path.clone()).map_err(|err| err.to_string())?;
        file.read_to_end(&mut buffer).map_err(|err| err.to_string())?;
        
        let content = String::from_utf8(buffer).map_err(|e| e.to_string())?;
        let parsed = Parser::new(&content)
            .into_offset_iter()
            .collect::<Vec<_>>();

        let formatted_content = self.format(parsed.iter(), "");

        if let Some(output_path) = output_path {
            let mut output_file = File::create(output_path).map_err(|err| err.to_string())?;
            write!(output_file, "{}", formatted_content).map_err(|err| err.to_string())?;
        } else {
            println!("{}", formatted_content);
        }

        Ok(())
    }

    fn format<'b>(
        &self,
        events: impl Iterator<Item = &'b (pulldown_cmark::Event<'b>, std::ops::Range<usize>)>,
        indent_level: &str,
    ) -> String {
        let mut result = String::new();
        let mut in_list = false;
        let mut in_paragraph = false;
        let mut in_blockquote = false;
        let mut current_indent = indent_level.to_string();
        let mut is_first_heading = true;
        let mut list_number = 1;
        let mut list_start = 1;
        let mut prev_was_list_item = false;
        let mut list_markers = Vec::new();

        for event in events {
            match &event.0 {
                pulldown_cmark::Event::Start(tag) => {
                    match tag {
                        Tag::Heading { level, .. } => {
                            let level_num = level.to_string().parse::<usize>().unwrap();
                            if level_num <= self.config.max_header_level as usize {
                                if !is_first_heading {
                                    result.push_str("\n\n");
                                }
                                result.push_str(&format!("{} ", "#".repeat(level_num)));
                                is_first_heading = false;
                            }
                        }
                        Tag::Paragraph => {
                            in_paragraph = true;
                            if !in_list && !in_blockquote {
                                result.push('\n');
                            }
                        }
                        Tag::BlockQuote(quote_type) => {
                            in_blockquote = true;
                            result.push_str("> ");
                        }
                        Tag::CodeBlock(info) => {
                            let info_str = match info {
                                pulldown_cmark::CodeBlockKind::Fenced(lang) if !lang.is_empty() => {
                                    format!("{}\n", lang)
                                }
                                _ => String::new(),
                            };
                            result.push_str(&format!("\n```{}\n", info_str));
                        }
                        Tag::List(Some(start)) => {
                            if !in_list {
                                result.push('\n');
                            }
                            in_list = true;
                            list_start = *start;
                            list_number = *start;
                            current_indent = " ".repeat(self.config.indent_size as usize);
                            list_markers.push(('o', *start));
                        }
                        Tag::List(None) => {
                            if !in_list {
                                result.push('\n');
                            }
                            in_list = true;
                            list_start = 1;
                            list_number = 1;
                            current_indent = " ".repeat(self.config.indent_size as usize);
                            list_markers.push(('u', 1));
                        }
                        Tag::Item => {
                            if in_list {
                                if let Some(&(marker_type, _)) = list_markers.last() {
                                    if marker_type == 'o' {
                                        let marker = if self.config.consecutive_numbering {
                                            format!("{}. ", list_number)
                                        } else {
                                            let pad = if list_number >= list_start + 10 {
                                                list_start.to_string().len()
                                            } else {
                                                0
                                            };
                                            format!("{:0width$}. ", list_number, width=pad)
                                        };
                                        if prev_was_list_item {
                                            result.push('\n');
                                        }
                                        result.push_str(&format!("{}{}", current_indent, marker));
                                        list_number += 1;
                                    } else {
                                        if prev_was_list_item {
                                            result.push('\n');
                                        }
                                        result.push_str(&format!("{}- ", current_indent));
                                    }
                                }
                                prev_was_list_item = true;
                            } else {
                                result.push_str("\n- ");
                                prev_was_list_item = true;
                            }
                        }
                        Tag::Emphasis => {
                            result.push('*');
                        }
                        Tag::Strong => {
                            result.push_str("**");
                        }
                        Tag::Link { dest_url, title, .. } => {
                            result.push('[');
                        }
                        Tag::Image { dest_url, title, .. } => {
                            result.push_str("![");
                        }
                        _ => {}
                    }
                }
                pulldown_cmark::Event::End(tag) => {
                    match tag {
                        TagEnd::Heading(_) => result.push('\n'),
                        TagEnd::Paragraph => {
                            in_paragraph = false;
                            if !in_list {
                                result.push('\n');
                            }
                        }
                        TagEnd::BlockQuote(quote_type) => {
                            in_blockquote = false;
                            result.push('\n');
                        }
                        TagEnd::CodeBlock => {
                            result.push_str("\n```\n");
                        }
                        TagEnd::List(_) => {
                            in_list = false;
                            list_markers.pop();
                            current_indent = "".to_string();
                            prev_was_list_item = false;
                            result.push('\n');
                        }
                        TagEnd::Emphasis => {
                            result.push('*');
                        }
                        TagEnd::Strong => {
                            result.push_str("**");
                        }
                        TagEnd::Link { .. } => {
                            result.push(']');
                        }
                        TagEnd::Image => {
                            result.push(']');
                        }
                        _ => {}
                    }
                }
                pulldown_cmark::Event::Text(text) => {
                    let text = if self.config.normalize_spaces {
                        text.trim()
                            .replace(char::is_whitespace, " ")
                            .replace("\\", "\\\\")
                            .replace("*", "\\*")
                            .replace("_", "\\_")
                            .replace("[", "\\[")
                            .replace("]", "\\]")
                            .replace("<", "\\<")
                            .replace("`", "\\`")
                    } else {
                        text.to_string()
                    };
                    result.push_str(&text);
                }
                pulldown_cmark::Event::Code(code) => {
                    result.push_str(&format!("`{}`", code));
                }
                pulldown_cmark::Event::SoftBreak => {
                    if in_paragraph {
                        result.push(' ');
                    } else {
                        result.push('\n');
                    }
                }
                pulldown_cmark::Event::HardBreak => {
                    result.push_str("\\\n");
                }
                pulldown_cmark::Event::Rule => {
                    result.push_str("\n---\n");
                }
                pulldown_cmark::Event::Html(html) => {
                    result.push_str(html);
                }
                pulldown_cmark::Event::InlineHtml(html) => {
                    result.push_str(html);
                }
                _ => {}
            }
        }

        // Post-processing
        if self.config.remove_extra_blank_lines {
            let lines: Vec<&str> = result.lines().collect();
            let mut filtered_lines = Vec::new();
            let mut prev_was_empty = false;

            for line in lines {
                let is_empty = line.trim().is_empty();
                if !is_empty || !prev_was_empty {
                    filtered_lines.push(line);
                }
                prev_was_empty = is_empty;
            }
            result = filtered_lines.join("\n");
        }

        if self.config.standardize_horizontal_lines {
            result = result.replace("***", "---")
                          .replace("___", "---")
                          .replace("***", "---");
        }

        // Ensure trailing newline
        if !result.ends_with('\n') {
            result.push('\n');
        }

        result
    }
}





// Структура для представления тестового случая
#[derive(Debug)]
struct TestCase {
    description: String,
    input: String,
    expected: String,
}

// Ошибка для агрегации всех неудавшихся тестов
#[derive(Debug)]
struct TestFailures {
    file_name: String,
    failures: Vec<TestFailure>,
}

#[derive(Debug)]
struct TestFailure {
    description: String,
    input: String,
    expected: String,
    actual: String,
}

impl fmt::Display for TestFailures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} failures in {}:", self.failures.len(), self.file_name)?;
        for (i, failure) in self.failures.iter().enumerate() {
            writeln!(f, "\nFailure {}: {}", i + 1, failure.description)?;
            writeln!(f, "Input:\n{}", failure.input)?;
            writeln!(f, "Expected:\n{}", failure.expected)?;
            writeln!(f, "Actual:\n{}", failure.actual)?;
        }
        Ok(())
    }
}

impl Error for TestFailures {}

// Функция для чтения тестовых файлов
fn read_fixture_file(path: &Path) -> Result<Vec<TestCase>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut tests = Vec::new();
    let mut section = 0;
    let mut last_pos = 0;
    let lines: Vec<&str> = text.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "." {
            if section == 0 {
                // Начало нового теста
                let description = if i > 0 { lines[i-1].trim() } else { "" };
                tests.push(TestCase {
                    description: description.to_string(),
                    input: String::new(),
                    expected: String::new(),
                });
                section = 1;
            } else if section == 1 {
                // Добавляем входные данные
                if let Some(test) = tests.last_mut() {
                    test.input = lines[last_pos+1..i].join("\n");
                }
                section = 2;
            } else if section == 2 {
                // Добавляем ожидаемый результат
                if let Some(test) = tests.last_mut() {
                    test.expected = lines[last_pos+1..i].join("\n");
                }
                section = 0;
            }
            last_pos = i;
        }
    }
    Ok(tests)
}

// Функция для запуска всех тестов из файла
fn run_tests_from_file(
    file_path: &Path,
    config: FormatterConfig,
) -> Result<(), TestFailures> {
    let file_name = file_path.file_name().unwrap().to_string_lossy().into_owned();
    let test_cases = read_fixture_file(file_path).unwrap();
    let formatter = MarkdownFormatter::new(&config);
    let mut failures = Vec::new();

    for test_case in test_cases {
        let actual = formatter.format_string(&test_case.input);
        if actual.trim() != test_case.expected.trim() {
            failures.push(TestFailure {
                description: test_case.description,
                input: test_case.input.clone(),
                expected: test_case.expected.clone(),
                actual,
            });
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(TestFailures {
            file_name,
            failures,
        })
    }
}

// Вспомогательная функция для форматирования строки
impl<'a> MarkdownFormatter<'a> {
    fn format_string(&self, input: &str) -> String {
        let parsed = Parser::new(input).into_offset_iter().collect::<Vec<_>>();
        self.format(parsed.iter(), "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Тест для файла с последовательной нумерацией
    #[test]
    fn test_consecutive_numbering() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("tests/data/consecutive_numbering.md");
        let config = FormatterConfig {
            consecutive_numbering: true,
            ..Default::default()
        };
        
        if let Err(e) = run_tests_from_file(&path, config) {
            panic!("{}", e);
        }
        Ok(())
    }

    // Тест для файла с настройками по умолчанию
    #[test]
    fn test_default_style() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("tests/data/default_style.md");
        let config = FormatterConfig::default();
        
        if let Err(e) = run_tests_from_file(&path, config) {
            panic!("{}", e);
        }
        Ok(())
    }

    // Тест для файла с переносом строк шириной 50
    #[test]
    fn test_wrap_width_50() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("tests/data/wrap_width_50.md");
        let config = FormatterConfig {
            wrap_width: 50,
            ..Default::default()
        };
        
        if let Err(e) = run_tests_from_file(&path, config) {
            panic!("{}", e);
        }
        Ok(())
    }

    // Общий тест для всех файлов
    #[test]
    fn test_all_files() -> Result<(), Box<dyn Error>> {
        let test_files = [
            ("consecutive_numbering.md", FormatterConfig {
                consecutive_numbering: true,
                ..Default::default()
            }),
            ("default_style.md", FormatterConfig::default()),
            ("wrap_width_50.md", FormatterConfig {
                wrap_width: 50,
                ..Default::default()
            }),
        ];

        let mut all_failures = Vec::new();

        for (file_name, config) in test_files {
            let path = PathBuf::from("tests").join(file_name);
            if let Err(e) = run_tests_from_file(&path, config) {
                all_failures.push(e);
            }
        }

        if !all_failures.is_empty() {
            let error_message = all_failures.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join("\n\n");
            panic!("Test failures:\n{}", error_message);
        }

        Ok(())
    }
}
