use chrono::NaiveDate;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::ProviderConfig;
use crate::events::{Category, Event};
use crate::providers::EventProvider;

#[derive(Debug, Clone, Copy)]
enum ProviderKind {
    Text,
    Csv,
}

#[derive(Debug)]
pub struct FileEventProvider {
    name: String,
    kind: ProviderKind,
    resource: PathBuf,
}

impl FileEventProvider {
    pub fn from_config(config: &ProviderConfig, base_dir: &Path) -> Result<Self, ProviderError> {
        let kind = ProviderKind::try_from(config.kind.as_str())?;
        let resource = base_dir.join(&config.resource);
        Ok(Self {
            name: config.name.clone(),
            kind,
            resource,
        })
    }

    fn load_text_events(&self) -> Result<Vec<Event>, ProviderError> {
        let mut events = Vec::new();
        let content = fs::read_to_string(&self.resource)?;
        for (idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let event = parse_text_line(trimmed, &self.name).map_err(|details| {
                ProviderError::InvalidFormat {
                    provider: self.name.clone(),
                    resource: self.resource.clone(),
                    details: format!("line {}: {details}", idx + 1),
                }
            })?;

            events.push(event);
        }

        Ok(events)
    }

    fn load_csv_events(&self) -> Result<Vec<Event>, ProviderError> {
        let mut events = Vec::new();
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&self.resource)?;

        for (idx, row) in reader.records().enumerate() {
            let row = row.map_err(ProviderError::Csv)?;
            if row.len() < 2 {
                return Err(ProviderError::InvalidFormat {
                    provider: self.name.clone(),
                    resource: self.resource.clone(),
                    details: format!("row {}: expected at least 2 columns", idx + 1),
                });
            }

            let date_col = row.get(0).unwrap_or("").trim();
            let desc_col = row.get(1).unwrap_or("").trim();
            let category_col = row.get(2).map(str::trim).filter(|value| !value.is_empty());

            if date_col.eq_ignore_ascii_case("date") && desc_col.eq_ignore_ascii_case("description")
            {
                continue;
            }
            if date_col.is_empty() && desc_col.is_empty() {
                continue;
            }

            let event =
                build_event(date_col, desc_col, category_col, &self.name).map_err(|details| {
                    ProviderError::InvalidFormat {
                        provider: self.name.clone(),
                        resource: self.resource.clone(),
                        details: format!("row {}: {details}", idx + 1),
                    }
                })?;
            events.push(event);
        }

        Ok(events)
    }
}

impl EventProvider for FileEventProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_events(&self, events: &mut Vec<Event>) -> Result<(), ProviderError> {
        let loaded = match self.kind {
            ProviderKind::Text => self.load_text_events()?,
            ProviderKind::Csv => self.load_csv_events()?,
        };
        events.extend(loaded);
        Ok(())
    }
}

#[derive(Debug)]
pub enum ProviderError {
    UnknownKind(String),
    Io(std::io::Error),
    Csv(csv::Error),
    InvalidFormat {
        provider: String,
        resource: PathBuf,
        details: String,
    },
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::UnknownKind(kind) => {
                write!(
                    f,
                    "unknown provider kind '{kind}', expected 'text' or 'csv'"
                )
            }
            ProviderError::Io(err) => write!(f, "file I/O error: {err}"),
            ProviderError::Csv(err) => write!(f, "CSV parse error: {err}"),
            ProviderError::InvalidFormat {
                provider,
                resource,
                details,
            } => write!(
                f,
                "invalid event format for provider '{}' in {}: {}",
                provider,
                resource.display(),
                details
            ),
        }
    }
}

impl Error for ProviderError {}

impl From<std::io::Error> for ProviderError {
    fn from(value: std::io::Error) -> Self {
        ProviderError::Io(value)
    }
}

impl From<csv::Error> for ProviderError {
    fn from(value: csv::Error) -> Self {
        ProviderError::Csv(value)
    }
}

impl TryFrom<&str> for ProviderKind {
    type Error = ProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_ascii_lowercase().as_str() {
            "text" => Ok(ProviderKind::Text),
            "csv" => Ok(ProviderKind::Csv),
            _ => Err(ProviderError::UnknownKind(value.to_string())),
        }
    }
}

fn parse_text_line(line: &str, default_category: &str) -> Result<Event, String> {
    let parts: Vec<&str> = line.split('|').map(str::trim).collect();
    if parts.len() < 2 {
        return Err(String::from(
            "expected 'YYYY-MM-DD|description' or 'YYYY-MM-DD|description|category'",
        ));
    }

    let category = parts.get(2).copied().filter(|value| !value.is_empty());
    build_event(parts[0], parts[1], category, default_category)
}

fn build_event(
    date_text: &str,
    description: &str,
    category_text: Option<&str>,
    default_category: &str,
) -> Result<Event, String> {
    let date = NaiveDate::parse_from_str(date_text, "%Y-%m-%d")
        .map_err(|err| format!("invalid date '{}': {}", date_text, err))?;
    if description.is_empty() {
        return Err(String::from("description cannot be empty"));
    }

    let category = match category_text {
        Some(value) => Category::from_str(value),
        None => Category::from_primary(default_category),
    };

    Ok(Event::new_singular(date, description.to_string(), category))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_text_line_uses_default_category_when_missing() {
        let event = parse_text_line("2020-03-11|WHO declared COVID-19 a pandemic", "history")
            .expect("line should parse");

        assert_eq!(event.year(), 2020);
        assert_eq!(event.category().to_string(), "history");
    }

    #[test]
    fn parse_text_line_uses_explicit_category_when_present() {
        let event = parse_text_line("2015-05-15|Rust 1.0 released|programming/rust", "history")
            .expect("line should parse");

        assert_eq!(event.year(), 2015);
        assert_eq!(event.category().to_string(), "programming/rust");
    }
}
