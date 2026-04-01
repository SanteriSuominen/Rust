use std::error::Error;
use std::path::Path;

pub mod birthday;
pub mod events;
pub mod filters;
pub mod providers;

use birthday::handle_birthday;
use chrono::{Datelike, Local, NaiveDate};
use serde::Deserialize;

use events::{Event, MonthDay};
use filters::FilterBuilder;
use providers::{EventProvider, SimpleProvider};
use crate::providers::{
    csvfile::CSVFileProvider,
    textfile::TextFileProvider,
    sqlite::SQLiteProvider,
    web::WebProvider,
};

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec::<ProviderConfig>,
}

fn create_providers(config: &Config, config_path: &Path) -> Vec::<Box<dyn EventProvider>> {
    // Try to create all the event providers specified in `config`.
    // Put them in a vector of trait objects.
    let mut providers: Vec::<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        match cfg.kind.as_str() {
            "text" => {
                let path = config_path.join(&cfg.resource);
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "csv" => {
                let path = config_path.join(&cfg.resource);
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "sqlite" => {
                let path = config_path.join(&cfg.resource);
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    let test_provider = SimpleProvider::new("test");
    providers.push(Box::new(test_provider));

    providers
}

pub fn run(config: &Config, config_path: &Path) -> Result<(), Box<dyn Error>> {
    handle_birthday();

    let mut events: Vec<Event> = Vec::new();
    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());

    let providers = create_providers(config, config_path);
    let filter = FilterBuilder::new()
    .month_day(today_month_day.clone())
    .category(crate::events::Category::from_str("work"))
    .text("Rust")
    .build();

    let mut count = 0;
    for provider in providers {
        provider.get_events(&filter, &mut events); // polymorphism!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'", 
            new_count - count,
            provider.name());
        count = new_count;
    }

    for event in events 
    {
        if today_month_day == event.month_day() 
        {
            println!("{}", event);
        }
    }

    Ok(())
}