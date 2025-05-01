use pulldown_cmark::{Options, Parser};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

// Структура для конфигурации форматирования
pub struct FormatterConfig {
    pub wrap_width: usize,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        FormatterConfig { wrap_width: 80 }
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
    fn format<'b>(&self, events: impl Iterator<Item = &'b (pulldown_cmark::Event<'b>, std::ops::Range<usize>)>,
                  indent_level: &str) -> String {
        let mut result = String::new();
        for event in events {
            match &event.0 {
                pulldown_cmark::Event::Start(tag) => {
                    result.push_str(&self.start_tag(tag));
                },
                pulldown_cmark::Event::End(_) => {},
                pulldown_cmark::Event::Text(text) => {
                    result.push_str(&self.text(text.to_string()));
                },
                pulldown_cmark::Event::Code(text) => {
                    result.push_str(&self.code(text.to_string()));
                },
                pulldown_cmark::Event::Html(html) => {
                    result.push_str(&self.html(html.to_string()));
                },
                pulldown_cmark::Event::InlineHtml(html) => {
                    result.push_str(&self.html(html.to_string()));
                },
                pulldown_cmark::Event::FootnoteReference(name) => {
                    result.push_str(&self.footnote_reference(name.to_string()));
                },
                pulldown_cmark::Event::SoftBreak => {
                    result.push('\n');
                },
                pulldown_cmark::Event::HardBreak => {
                    result.push_str("<br>\n");
                },
                pulldown_cmark::Event::Rule => {
                    result.push_str("---");
                },
                pulldown_cmark::Event::TaskListMarker(is_checked) => {
                    result.push_str(if *is_checked { "[x]" } else { "[ ]" });
                },
                pulldown_cmark::Event::InlineMath(math) => {
                    result.push_str(&format!("${}$", math));
                },
                pulldown_cmark::Event::DisplayMath(math) => {
                    result.push_str(&format!("$${}$$", math));
                },
            };
        }
        result
    }

    // Вспомогательные методы для различных типов тегов и элементов

    fn start_tag(&self, tag: &pulldown_cmark::Tag<'_>) -> String {
        match tag {
            pulldown_cmark::Tag::Heading { level, .. } => format!("{}\n{}", "#".repeat(*level as usize), ""),
            pulldown_cmark::Tag::BlockQuote(..) => String::new(),
            pulldown_cmark::Tag::CodeBlock(info) => format!("\n```{:?}\n", info),
            pulldown_cmark::Tag::Item => "".to_string(),
            pulldown_cmark::Tag::Table(_) => "".to_string(),
            pulldown_cmark::Tag::TableHead => "".to_string(),
            pulldown_cmark::Tag::TableRow => "".to_string(),
            pulldown_cmark::Tag::TableCell => "".to_string(),
            pulldown_cmark::Tag::Emphasis => "_".to_string(),
            pulldown_cmark::Tag::Strong => "__".to_string(),
            pulldown_cmark::Tag::Link { dest_url: _, title: _, id: _, link_type: _ } => "".to_string(),
            pulldown_cmark::Tag::Image { dest_url: _, title: _, id: _, link_type: _ } => "".to_string(),
            pulldown_cmark::Tag::FootnoteDefinition(_) => "".to_string(),
            pulldown_cmark::Tag::Paragraph => "".to_string(),
            pulldown_cmark::Tag::List(_) => "".to_string(),
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
        text
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

fn main() -> Result<(), String> {
    let config = FormatterConfig::default();
    let formatter = MarkdownFormatter::new(&config);

    let input_path = PathBuf::from("input.md"); // Подставьте реальный путь к вашему файлу
    let output_path = Some(PathBuf::from("output.md")); // Опциональный выходной файл

    formatter.format_markdown(input_path, output_path)?;

    Ok(())
}