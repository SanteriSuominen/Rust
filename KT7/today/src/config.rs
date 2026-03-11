use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_CONFIG_PATH: &str = "config/config.toml";

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
        Self::load_from_path(Path::new(DEFAULT_CONFIG_PATH))
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
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

pub fn default_config_path() -> PathBuf {
    PathBuf::from(DEFAULT_CONFIG_PATH)
}
