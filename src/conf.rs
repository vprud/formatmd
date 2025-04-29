use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use toml::Value;

const VALID_WRAP_VALUES: &[&str] = &["keep", "no"];
const VALID_EOL_VALUES: &[&str] = &["crlf", "lf", "keep"];
const VALID_KEYS: &[&str] = &[
    "wrap",
    "number",
    "end_of_line",
    "validate",
    "exclude",
    "plugin",
    "extensions",
    "codeformatters",
];

// Возможные варианты опций
pub struct FormatOpts {
    pub wrap: WrapOption,
    pub number: bool,
    pub end_of_line: EndOfLine,
    pub validate: bool,
    pub exclude: Vec<String>,
    pub plugin: HashMap<String, String>,
    pub extensions: Option<Vec<String>>,
    pub codeformatters: Option<Vec<String>>,
}

// Варианты поведения wrap
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrapOption {
    Keep,
    No,
    Length(u32),
}

// Возможные символы завершения строки
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndOfLine {
    Lf,
    CrLf,
    Keep,
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError {
            message: error.to_string(),
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError {
            message: error.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, ConfigError>;

pub fn read_toml_opts(conf_dir: &Path) -> Result<(FormatOpts, Option<PathBuf>)> {
    // Константа для пустых настроек
    let opts = FormatOpts {
        wrap: WrapOption::Keep,
        number: false,
        end_of_line: EndOfLine::Lf,
        validate: true,
        exclude: Vec::new(),
        plugin: HashMap::new(),
        extensions: None,
        codeformatters: None,
    };

    let conf_path = conf_dir.join(".mdformat.toml");
    if !conf_path.exists() || !conf_path.is_file() {
        return Ok((opts, None));
    }

    let content = fs::read_to_string(&conf_path)?;
    let toml_opts: Value = toml::from_str(&content)?;

    validate_config(&toml_opts, &conf_path)?;

    Ok((opts, Some(conf_path)))
}

fn validate_config(config: &Value, conf_path: &Path) -> Result<()> {
    for key in config
        .as_table()
        .ok_or_else(|| ConfigError {
            message: format!("Config at {:?} is not valid TOML table.", conf_path),
        })?
        .keys()
    {
        if !VALID_KEYS.contains(&key.as_str()) {
            return Err(ConfigError {
                message: format!(
                    "Invalid key '{key}' found in {:?}. Valid keys are {:?}",
                    conf_path, VALID_KEYS
                ),
            });
        }
    }

    if let Some(value) = config.get("wrap") {
        if !(value.is_integer() && value.as_integer().map(|v| v > 1).unwrap_or(false))
            && !VALID_WRAP_VALUES.contains(&value.as_str().unwrap())
        {
            return Err(ConfigError {
                message: format!("Invalid 'wrap' value in {:?}", conf_path),
            });
        }
    }

    if let Some(value) = config.get("end_of_line") {
        if !VALID_EOL_VALUES.contains(&value.as_str().unwrap()) {
            return Err(ConfigError {
                message: format!("Invalid 'end_of_line' value in {:?}", conf_path),
            });
        }
    }

    if let Some(value) = config.get("validate") {
        if !value.is_bool() {
            return Err(ConfigError {
                message: format!("Invalid 'validate' value in {:?}", conf_path),
            });
        }
    }

    if let Some(value) = config.get("number") {
        if !value.is_bool() {
            return Err(ConfigError {
                message: format!("Invalid 'number' value in {:?}", conf_path),
            });
        }
    }

    if let Some(exclude_list) = config.get("exclude") {
        if exclude_list.is_array() {
            for item in exclude_list.as_array().unwrap() {
                if !item.is_str() {
                    return Err(ConfigError {
                        message: format!(
                            "All items in 'exclude' must be strings in {:?}",
                            conf_path
                        ),
                    });
                }
            }
        } else {
            return Err(ConfigError {
                message: format!("The 'exclude' field must be an array in {:?}", conf_path),
            });
        }
    }

    if let Some(plugins) = config.get("plugin") {
        if !plugins.is_table() {
            return Err(ConfigError {
                message: format!("The 'plugin' field must be a map in {:?}", conf_path),
            });
        }
    }

    if let Some(exts) = config.get("extensions") {
        if exts.is_array() {
            for item in exts.as_array().unwrap() {
                if !item.is_str() {
                    return Err(ConfigError {
                        message: format!(
                            "All items in 'extensions' must be strings in {:?}",
                            conf_path
                        ),
                    });
                }
            }
        } else {
            return Err(ConfigError {
                message: format!("The 'extensions' field must be an array in {:?}", conf_path),
            });
        }
    }

    if let Some(codeformatters) = config.get("codeformatters") {
        if codeformatters.is_array() {
            for item in codeformatters.as_array().unwrap() {
                if !item.is_str() {
                    return Err(ConfigError {
                        message: format!(
                            "All items in 'codeformatters' must be strings in {:?}",
                            conf_path
                        ),
                    });
                }
            }
        } else {
            return Err(ConfigError {
                message: format!(
                    "The 'codeformatters' field must be an array in {:?}",
                    conf_path
                ),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_validate_config_valid() {
        let valid_config = r#"
            wrap = "keep"
            number = true
            end_of_line = "lf"
            validate = true
            exclude = ["file1.md", "file2.md"]
            plugin = { "key1" = "value1", "key2" = "value2" }
            extensions = ["md", "markdown"]
            codeformatters = ["rustfmt", "prettier"]
        "#;
        let config: Value = toml::from_str(valid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        assert!(validate_config(&config, conf_path).is_ok());
    }

    #[test]
    fn test_validate_config_invalid_key() {
        let invalid_config = r#"
            invalid_key = "value"
        "#;
        let config: Value = toml::from_str(invalid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        let result = validate_config(&config, conf_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid key"));
    }

    #[test]
    fn test_validate_config_invalid_wrap() {
        let invalid_config = r#"
            wrap = "invalid"
        "#;
        let config: Value = toml::from_str(invalid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        let result = validate_config(&config, conf_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid 'wrap' value")
        );
    }

    #[test]
    fn test_validate_config_invalid_end_of_line() {
        let invalid_config = r#"
            end_of_line = "invalid"
        "#;
        let config: Value = toml::from_str(invalid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        let result = validate_config(&config, conf_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid 'end_of_line' value")
        );
    }

    #[test]
    fn test_validate_config_invalid_exclude() {
        let invalid_config = r#"
            exclude = [123, "file2.md"]
        "#;
        let config: Value = toml::from_str(invalid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        let result = validate_config(&config, conf_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("All items in 'exclude' must be strings")
        );
    }

    #[test]
    fn test_validate_config_invalid_plugin() {
        let invalid_config = r#"
            plugin = "not_a_map"
        "#;
        let config: Value = toml::from_str(invalid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        let result = validate_config(&config, conf_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("The 'plugin' field must be a map")
        );
    }

    #[test]
    fn test_validate_config_invalid_extensions() {
        let invalid_config = r#"
            extensions = ["md", 123]
        "#;
        let config: Value = toml::from_str(invalid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        let result = validate_config(&config, conf_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("All items in 'extensions' must be strings")
        );
    }

    #[test]
    fn test_validate_config_invalid_codeformatters() {
        let invalid_config = r#"
            codeformatters = ["rustfmt", 123]
        "#;
        let config: Value = toml::from_str(invalid_config).unwrap();
        let conf_path = Path::new("test_config.toml");
        let result = validate_config(&config, conf_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("All items in 'codeformatters' must be strings")
        );
    }
}
