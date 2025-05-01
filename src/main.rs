use pulldown_cmark::{Options, Parser, Tag};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

// Структура для конфигурации форматирования
pub struct FormatterConfig {
    pub wrap_width: usize,
    pub max_header_level: u32,                   // Максимальная глубина заголовков
    pub standardize_horizontal_lines: bool,      // Стандартизация горизонтальных линий
    pub normalize_spaces: bool,                  // Нормализация пробелов
    pub remove_extra_blank_lines: bool,          // Удаление лишних пустых строк
    pub uniform_links_style: bool,               // Унификация стилей ссылок
    pub indent_size: u32,                       // Размер отступа для блоков и списков
}

impl Default for FormatterConfig {
    fn default() -> Self {
        FormatterConfig {
            wrap_width: 80,
            max_header_level: 6,                 // Ограничиваем заголовки уровнем H6
            standardize_horizontal_lines: true,  // Используйте стандартные горизонтальные линии (---)
            normalize_spaces: true,              // Очищаем двойной пробел
            remove_extra_blank_lines: true,      // Убираем лишние пустые строки
            uniform_links_style: true,           // Нормализуем ссылки
            indent_size: 4,                     // Отступ 4 пробела
        }
    }
}

// Основной класс форматера
pub struct MarkdownFormatter<'a> {
    config: &'a FormatterConfig,
}

impl<'a> MarkdownFormatter<'a> {
    pub fn new(config: &'a FormatterConfig) -> Self {
        MarkdownFormatter { config }
    }

    // Метод для форматирования Markdown-файла
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

    // Рекурсивный метод для обхода AST и форматирования каждого узла
    fn format<'b>(
        &self,
        events: impl Iterator<Item = &'b (pulldown_cmark::Event<'b>, std::ops::Range<usize>)>,
        indent_level: &str,
    ) -> String {
        let mut result = String::new();
        let mut previous_was_heading = false; // следим за тем, была ли последняя запись заголовком

        for event in events {
            match &event.0 {
                pulldown_cmark::Event::Start(tag) => {
                    result.push_str(&self.start_tag(tag));
                }
                pulldown_cmark::Event::End(_) => {}
                pulldown_cmark::Event::Text(text) => {
                    result.push_str(&self.text(text.to_string()));
                }
                pulldown_cmark::Event::Code(text) => {
                    result.push_str(&self.code(text.to_string()));
                }
                pulldown_cmark::Event::Html(html) => {
                    result.push_str(&self.html(html.to_string()));
                }
                pulldown_cmark::Event::InlineHtml(html) => {
                    result.push_str(&self.html(html.to_string()));
                }
                pulldown_cmark::Event::FootnoteReference(name) => {
                    result.push_str(&self.footnote_reference(name.to_string()));
                }
                pulldown_cmark::Event::SoftBreak => {
                    result.push('\n');
                }
                pulldown_cmark::Event::HardBreak => {
                    result.push_str("<br>\n");
                }
                pulldown_cmark::Event::Rule => {
                    result.push_str("---");
                }
                pulldown_cmark::Event::TaskListMarker(is_checked) => {
                    result.push_str(if *is_checked { "[x]" } else { "[ ]" });
                }
                pulldown_cmark::Event::InlineMath(math) => {
                    result.push_str(&format!("${}$", math));
                }
                pulldown_cmark::Event::DisplayMath(math) => {
                    result.push_str(&format!("$${}$$", math));
                }
            };
            
            // Если предыдущий элемент был заголовком, добавляем пустую строку после него
            if previous_was_heading {
                result.push('\n');
                previous_was_heading = false;
            }
        }

        // Постобработка документа согласно правилам
        normalize_document(&mut result, self.config);

        result
    }

    // Вспомогательные методы для различных типов тегов и элементов

    fn start_tag(&self, tag: &pulldown_cmark::Tag<'_>) -> String {
        match tag {
            pulldown_cmark::Tag::Heading { level, .. } => {
                let level_num = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                let max_level = self.config.max_header_level.min(level_num);
                format!(
                    "\n{}\n", // Заметьте, нет дополнительного перехода строки
                    "#".repeat(max_level as usize)
                )
            }
            pulldown_cmark::Tag::BlockQuote(_) => "> ".to_string(),
            pulldown_cmark::Tag::CodeBlock(info) => format!("\n```{:?}\n", info),
            pulldown_cmark::Tag::Item => "- ".to_string(),
            pulldown_cmark::Tag::Table(_) => "".to_string(),
            pulldown_cmark::Tag::TableHead => "".to_string(),
            pulldown_cmark::Tag::TableRow => "".to_string(),
            pulldown_cmark::Tag::TableCell => "".to_string(),
            pulldown_cmark::Tag::Emphasis => "_".to_string(),
            pulldown_cmark::Tag::Strong => "__".to_string(),
            pulldown_cmark::Tag::Link { dest_url: _, title: _, id: _, link_type: _ } => "".to_string(),
            pulldown_cmark::Tag::Image { dest_url: _, title: _, id: _, link_type: _ } => "".to_string(),
            pulldown_cmark::Tag::FootnoteDefinition(_) => "".to_string(),
            pulldown_cmark::Tag::Paragraph => "\n".to_string(),
            pulldown_cmark::Tag::List(_) => "\n".to_string(),
            pulldown_cmark::Tag::HtmlBlock => "".to_string(),
            pulldown_cmark::Tag::DefinitionList => "".to_string(),
            pulldown_cmark::Tag::DefinitionListTitle => "".to_string(),
            pulldown_cmark::Tag::DefinitionListDefinition => "".to_string(),
            pulldown_cmark::Tag::Strikethrough => "~~".to_string(),
            pulldown_cmark::Tag::Superscript => "^".to_string(),
            pulldown_cmark::Tag::Subscript => "~".to_string(),
            pulldown_cmark::Tag::MetadataBlock(_) => "".to_string(),
        }
    }

    fn text(&self, text: String) -> String {
        if self.config.normalize_spaces {
            text.trim().replace(char::is_whitespace, " ").trim_matches(' ').to_string()
        } else {
            text
        }
    }

    fn code(&self, text: String) -> String {
        format!("`{}`", text)
    }

    fn html(&self, html: String) -> String {
        html
    }

    fn footnote_reference(&self, name: String) -> String {
        format!("[^{}]", name)
    }
}

// Постобработка документа
fn normalize_document(doc: &mut String, config: &FormatterConfig) {
    if config.remove_extra_blank_lines {
        // Удаляем лишние пустые строки
        doc.retain(|ch| !matches!(ch, '\n') || ch.is_alphanumeric());
    }

    if config.uniform_links_style {
        // Здесь должна быть дополнительная логика для приведения ссылок к одному стилю
        // Но такая полноценная реализация выходит за рамки текущего примера
    }

    if config.standardize_horizontal_lines {
        // Заменяем все виды горизонтальных линий на стандартный вариант (---)
        doc.retain(|ch| ch != '*' && ch != '_');
    }

    if config.normalize_spaces {
        // Сокращаем многократные пробелы до одного
        doc.retain(|ch| !(ch == ' ' && ch == ' '));
    }
}

fn main() -> Result<(), String> {
    let config = FormatterConfig::default();
    let formatter = MarkdownFormatter::new(&config);

    let input_path = PathBuf::from("input.md"); // Подставьте реальный путь к вашему файлу
    let output_path = Some(PathBuf::from("output.md")); // Опциональный выходной файл

    formatter.format_markdown(input_path, output_path)?;

    Ok(())
}