<<<<<<< HEAD
fn main() {
    if let Err(e) = today::run() {
        eprintln!("Error: {}", e);
        return;
    }
}
=======
use chrono::{Datelike, Local};
use std::path::Path;
use today::config::{AppConfig, config_file_path};
use today::events::{Event, MonthDay};
use today::providers::{EventProvider, FileEventProvider};

fn main() {
    let config_path = match config_file_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to resolve config path: {}", err);
            return;
        }
    };
    let config = match AppConfig::load_default() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load config '{}': {}", config_path.display(), err);
            return;
        }
    };

    let base_dir = config_path.parent().unwrap_or(Path::new("."));
    let mut events: Vec<Event> = Vec::new();
    for provider_config in &config.providers {
        let provider = match FileEventProvider::from_config(provider_config, base_dir) {
            Ok(provider) => provider,
            Err(err) => {
                eprintln!("Skipping provider '{}': {}", provider_config.name, err);
                continue;
            }
        };

        println!("Loading provider '{}'", provider.name());
        if let Err(err) = provider.get_events(&mut events) {
            eprintln!("Failed loading events from '{}': {}", provider.name(), err);
        }
    }

    let today = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());
    let mut todays_events: Vec<Event> = events
        .into_iter()
        .filter(|event| event.month_day() == today_month_day)
        .collect();

    if todays_events.is_empty() {
        println!("No events for today.");
        return;
    }

    todays_events.sort_by_key(|event| event.year());
    for event in todays_events {
        println!("{event}");
    }
}
>>>>>>> 24dd8ea67d97c113c73747eb2ab7bb906304006e
