use std::collections::HashMap;

// Представление плагин-интерфейса
pub trait ParserExtension {
    // TODO: добавить разные трейты для мутации и без с AST
    // Удалено CHANGES_AST, так как оно несовместимо с dyn

    // Тип рендереров для узлов AST
    fn renderers(&self) -> HashMap<String, Box<dyn Fn(&String) -> String>>;

    // Постпроцессоры для вывода (опционально)
    fn postprocessors(&self) -> Option<HashMap<String, Box<dyn Fn(String) -> String>>> {
        None
    }

    // Добавляет аргументы командной строки
    fn add_cli_options(&self, _parser: &mut clap::ArgMatches) {}

    // Обрабатывает группу аргументов CLI
    fn add_cli_argument_group(&self, _group: &mut clap::ArgGroup) {}

    // Применяет изменения к объекту Markdown-парсера
    fn update_mdit(&self, _mdit: &mut markdown::message::Message) {}
}

struct PluginRegistry {
    code_formatters: HashMap<String, Box<dyn Fn(&str, &str) -> String>>,
    parser_extensions: HashMap<String, Box<dyn ParserExtension>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            code_formatters: HashMap::new(),
            parser_extensions: HashMap::new(),
        }
    }

    pub fn register_code_formatter(
        &mut self,
        name: String,
        formatter: impl Fn(&str, &str) -> String + 'static,
    ) {
        self.code_formatters.insert(name, Box::new(formatter));
    }

    pub fn register_parser_extension(
        &mut self,
        name: String,
        extension: impl ParserExtension + 'static,
    ) {
        self.parser_extensions.insert(name, Box::new(extension));
    }
}

struct MyPlugin;

impl ParserExtension for MyPlugin {
    fn renderers(&self) -> HashMap<String, Box<dyn Fn(&String) -> String>> {
        HashMap::new()
    }
}

fn load_plugins(registry: &mut PluginRegistry) {
    // Загрузка форматов и расширений пока выполняется вручную
    registry.register_code_formatter("rustfmt".to_string(), |code, lang| {
        format!("Formatted {} code using {}", lang, "rustfmt")
    });

    let extension = MyPlugin;
    registry.register_parser_extension("my-plugin".to_string(), extension);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_load_plugins() {
        let mut registry = PluginRegistry::new();
        load_plugins(&mut registry);

        assert!(registry.code_formatters.contains_key("rustfmt"));
        assert!(registry.parser_extensions.contains_key("my-plugin"));
    }

    #[test]
    fn test_code_formatter() {
        let mut registry = PluginRegistry::new();
        registry.register_code_formatter("rustfmt".to_string(), |code, lang| {
            format!("{} formatted", lang)
        });

        let result = registry.code_formatters.get("rustfmt").unwrap()(/* some input */ "", "");
        assert_eq!(result, " formatted");
    }
}
