use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const APP_DIR_NAME: &str = "today";
const CONFIG_FILE_NAME: &str = "config.toml";
const DEFAULT_CONFIG_TOML: &str = r#"[[providers]]
name = "history"
kind = "csv"
resource = "history.csv"

[[providers]]
name = "programming"
kind = "text"
resource = "programming.txt"

[[providers]]
name = "computing"
kind = "text"
resource = "computing.txt"
"#;
const DEFAULT_HISTORY_CSV: &str = r#"date,description,category
2020-03-11,WHO declared COVID-19 a pandemic,history/world
1989-11-09,Berlin Wall fall announced,history/europe
1969-07-20,Apollo 11 Moon landing,science/space
"#;
const DEFAULT_PROGRAMMING_TXT: &str = r#"# Format: YYYY-MM-DD|description|optional category
2015-05-15|Rust 1.0.0 released|programming/rust
1994-01-03|Python 1.0.0 released|programming/python
2009-11-10|Go announced by Google|programming/go
"#;
const DEFAULT_COMPUTING_TXT: &str = r#"# Format: YYYY-MM-DD|description|optional category
1991-08-25|Linux announced by Linus Torvalds|computing/linux
1994-03-14|Linux kernel 1.0.0 released|computing/linux
1977-04-16|Apple II introduced|computing/hardware
"#;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    pub resource: String,
}

impl AppConfig {
    pub fn load_default() -> Result<Self, ConfigError> {
        let path = config_file_path()?;
        if !path.exists() {
            write_default_files(&path)?;
        }
        Self::load_from_path(&path)
    }

    pub fn load_from_path(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let mut config: AppConfig = toml::from_str(&content)?;
        config.providers.retain(|provider| {
            !provider.name.trim().is_empty()
                && !provider.kind.trim().is_empty()
                && !provider.resource.trim().is_empty()
        });
        Ok(config)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    ConfigDirUnavailable,
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ConfigDirUnavailable => {
                write!(f, "failed to resolve OS config directory")
            }
            ConfigError::Io(err) => write!(f, "failed to read config file: {err}"),
            ConfigError::Parse(err) => write!(f, "failed to parse TOML config: {err}"),
        }
    }
}

impl Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::Io(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        ConfigError::Parse(value)
    }
}

pub fn config_file_path() -> Result<PathBuf, ConfigError> {
    let config_dir = dirs::config_dir().ok_or(ConfigError::ConfigDirUnavailable)?;
    Ok(config_dir.join(APP_DIR_NAME).join(CONFIG_FILE_NAME))
}

fn write_default_files(config_path: &Path) -> Result<(), ConfigError> {
    let config_dir = config_path
        .parent()
        .ok_or(ConfigError::ConfigDirUnavailable)?;
    fs::create_dir_all(config_dir)?;

    write_if_missing(config_path, DEFAULT_CONFIG_TOML)?;
    write_if_missing(&config_dir.join("history.csv"), DEFAULT_HISTORY_CSV)?;
    write_if_missing(&config_dir.join("programming.txt"), DEFAULT_PROGRAMMING_TXT)?;
    write_if_missing(&config_dir.join("computing.txt"), DEFAULT_COMPUTING_TXT)?;

    Ok(())
}

fn write_if_missing(path: &Path, content: &str) -> Result<(), ConfigError> {
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}
