use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub mod birthday;
pub mod events;
pub mod filters;
pub mod providers;

use birthday::handle_birthday;
use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;

use crate::providers::{
    csvfile::CSVFileProvider, sqlite::SQLiteProvider, textfile::TextFileProvider, web::WebProvider,
};
use events::{Category, Event, MonthDay};
use filters::FilterBuilder;
use providers::EventProvider;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    pub all: bool,
    pub no_birthday: bool,
    pub month_day: Option<MonthDay>,
    pub category: Option<Category>,
    pub text: Option<String>,
    pub excluded_categories: Vec<Category>,
}

fn create_providers(config: &Config, config_path: &Path) -> Vec<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        match cfg.kind.as_str() {
            "text" => {
                let path = config_path.join(&cfg.resource);
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            }
            "csv" => {
                let path = config_path.join(&cfg.resource);
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            }
            "sqlite" => {
                let path = config_path.join(&cfg.resource);
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            }
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            }
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    providers
}

pub fn parse_month_day_input(input: &str) -> Result<MonthDay, String> {
    let compact = input.replace('-', "");
    if compact.len() != 4 || !compact.chars().all(|ch| ch.is_ascii_digit()) {
        return Err("expected MM-DD or MMDD".to_string());
    }

    let month: u32 = compact[..2]
        .parse()
        .map_err(|_| "month should be two digits".to_string())?;
    let day: u32 = compact[2..]
        .parse()
        .map_err(|_| "day should be two digits".to_string())?;

    if NaiveDate::from_ymd_opt(2024, month, day).is_none() {
        return Err("invalid month/day value".to_string());
    }

    Ok(MonthDay::new(month, day))
}

pub fn parse_event_date_input(input: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(input, "%F").map_err(|_| "expected YYYY-MM-DD".to_string())
}

pub fn list_provider_names(config: &Config) -> Vec<String> {
    config
        .providers
        .iter()
        .map(|provider| provider.name.clone())
        .collect()
}

fn append_text_event(
    path: &Path,
    date: NaiveDate,
    description: &str,
    category: &Category,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", date.format("%F"))?;
    writeln!(file, "{}", description)?;
    writeln!(file, "{}", category)?;
    writeln!(file, "---")?;
    Ok(())
}

fn append_csv_event(
    path: &Path,
    date: NaiveDate,
    description: &str,
    category: &Category,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new().create(true).append(true).open(path)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    writer.write_record([
        date.format("%F").to_string(),
        description.to_string(),
        category.to_string(),
    ])?;
    writer.flush()?;
    Ok(())
}

pub fn add_event_to_provider(
    config: &Config,
    config_path: &Path,
    provider_name: &str,
    date: NaiveDate,
    description: &str,
    category: &Category,
) -> Result<(), Box<dyn Error>> {
    let provider = config
        .providers
        .iter()
        .find(|provider| provider.name == provider_name)
        .ok_or_else(|| format!("provider '{}' not found in configuration", provider_name))?;
    let resource_path = config_path.join(&provider.resource);

    match provider.kind.as_str() {
        "text" => append_text_event(&resource_path, date, description, category),
        "csv" => append_csv_event(&resource_path, date, description, category),
        "sqlite" => Err("adding events to sqlite providers is not implemented".into()),
        "web" => Err("web providers are read-only".into()),
        _ => Err(format!("unsupported provider kind '{}'", provider.kind).into()),
    }
}

pub fn run(
    config: &Config,
    config_path: &Path,
    options: &RunOptions,
) -> Result<(), Box<dyn Error>> {
    if !options.no_birthday {
        handle_birthday();
    }

    let mut events: Vec<Event> = Vec::new();
    let providers = create_providers(config, config_path);
    let mut filter_builder = FilterBuilder::new();

    if !options.all {
        let month_day = options.month_day.clone().unwrap_or_else(|| {
            let today = Local::now().date_naive();
            MonthDay::new(today.month(), today.day())
        });
        filter_builder = filter_builder.month_day(month_day);
    }

    if let Some(category) = options.category.clone() {
        filter_builder = filter_builder.category(category);
    }
    if let Some(text) = options.text.as_ref() {
        filter_builder = filter_builder.text(text);
    }
    if !options.excluded_categories.is_empty() {
        filter_builder = filter_builder.exclude_categories(options.excluded_categories.clone());
    }
    let filter = filter_builder.build();

    let mut count = 0;
    for provider in providers {
        provider.get_events(&filter, &mut events); // polymorphism!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'",
            new_count - count,
            provider.name()
        );
        count = new_count;
    }

    for event in events {
        println!("{}", event);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_month_day_input;
    use crate::events::MonthDay;

    #[test]
    fn parses_mmdd() {
        assert_eq!(parse_month_day_input("0402").unwrap(), MonthDay::new(4, 2));
    }

    #[test]
    fn parses_mm_dash_dd() {
        assert_eq!(parse_month_day_input("04-02").unwrap(), MonthDay::new(4, 2));
    }

    #[test]
    fn rejects_bad_format() {
        assert!(parse_month_day_input("4-2").is_err());
        assert!(parse_month_day_input("040").is_err());
        assert!(parse_month_day_input("ab-cd").is_err());
    }

    #[test]
    fn rejects_invalid_date() {
        assert!(parse_month_day_input("13-02").is_err());
        assert!(parse_month_day_input("02-30").is_err());
    }
}
