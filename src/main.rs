use pulldown_cmark::{Options, Parser, Tag, TagEnd};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub struct FormatterConfig {
    pub wrap_width: usize,
    pub max_header_level: u32,
    pub standardize_horizontal_lines: bool,
    pub normalize_spaces: bool,
    pub remove_extra_blank_lines: bool,
    pub uniform_links_style: bool,
    pub indent_size: u32,
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
        let mut current_indent = indent_level.to_string();
        let mut is_first_heading = true;

        for event in events {
            match &event.0 {
                pulldown_cmark::Event::Start(tag) => {
                    match tag {
                        Tag::Heading { level, .. } => {
                            let level_num = (*level as u8) as usize;
                            if level_num <= self.config.max_header_level as usize {
                                if !is_first_heading {
                                    result.push('\n');
                                }
                                result.push_str(&format!("{} ", "#".repeat(level_num)));
                                is_first_heading = false;
                            }
                        }
                        Tag::Paragraph => {
                            in_paragraph = true;
                            if !in_list {
                                result.push('\n');
                            }
                        }
                        Tag::List(Some(1)) => {
                            in_list = true;
                            current_indent = " ".repeat(self.config.indent_size as usize);
                        }
                        Tag::List(None) => {
                            in_list = true;
                            current_indent = " ".repeat(self.config.indent_size as usize);
                        }
                        Tag::Item => {
                            if in_list {
                                result.push_str(&format!("\n{}- ", &current_indent[..current_indent.len()-2]));
                            } else {
                                result.push_str("- ");
                            }
                        }
                        _ => {}
                    }
                }
                pulldown_cmark::Event::End(tag) => {
                    match tag {
                        TagEnd::Heading(_) => result.push('\n'),
                        TagEnd::Paragraph => {
                            in_paragraph = false;
                            result.push('\n');
                        }
                        TagEnd::List(_) => {
                            in_list = false;
                            current_indent = "".to_string();
                            result.push('\n');
                        }
                        _ => {}
                    }
                }
                pulldown_cmark::Event::Text(text) => {
                    let text = if self.config.normalize_spaces {
                        text.trim().replace(char::is_whitespace, " ")
                    } else {
                        text.to_string()
                    };
                    result.push_str(&text);
                }
                pulldown_cmark::Event::SoftBreak => {
                    if in_paragraph {
                        result.push(' ');
                    } else {
                        result.push('\n');
                    }
                }
                pulldown_cmark::Event::HardBreak => {
                    result.push_str("<br>\n");
                }
                pulldown_cmark::Event::Rule => {
                    result.push_str("\n---\n");
                }
                _ => {}
            }
        }

        result
    }
}


fn main() -> Result<(), String> {
    let config = FormatterConfig::default();
    let formatter = MarkdownFormatter::new(&config);

    let input_path = PathBuf::from("input.md");
    let output_path = Some(PathBuf::from("output.md"));

    formatter.format_markdown(input_path, output_path)?;

    Ok(())
}