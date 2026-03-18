use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use today::{Config, run};

const APP_NAME: &str = "today";
const CONFIG_FILE_NAME: &str = "today.toml";
const DEFAULT_CONFIG: &str = r#"[[providers]]
name = "text-events"
kind = "text"
resource = "resources/events.txt"

[[providers]]
name = "csv-events"
kind = "csv"
resource = "resources/events.csv"
"#;
const DEFAULT_TEXT_RESOURCE: &str = "\
2026-01-01
New Year's Day
holiday
---
";
const DEFAULT_CSV_RESOURCE: &str = "2026-01-02,Sample CSV event,holiday\n";

fn main() {
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            if let Err(err) = ensure_default_files(&path) {
                eprintln!("Unable to initialize config/resources: {}", err);
                return;
            }

            let toml_path = path.join(CONFIG_FILE_NAME);
            let config_str = match fs::read_to_string(&toml_path) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!(
                        "Unable to read configuration file '{}': {}",
                        toml_path.display(),
                        err
                    );
                    return;
                }
            };

            let config: Config = match toml::from_str(&config_str) {
                Ok(value) => value,
                Err(err) => {
                    eprintln!(
                        "Unable to parse configuration file '{}': {}",
                        toml_path.display(),
                        err
                    );
                    return;
                }
            };

            if let Err(e) = run(&config, &path) {
                eprintln!("Error: {}", e);
                return;
            }
        },
        None => {
            eprintln!("Unable to configure the application");
            return;
        }
    }
}

// Gets the configuration directory path for the application
// named in the `app_name` argument.
// If the directory does not exist, tries to create it.
// Returns an optional `PathBuf` containing the directory path,
// or None if the directory can't be created.
fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join(app_name);

        if !config_path.exists() {
            if let Err(_) = fs::create_dir_all(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            }
        }

        return Some(config_path);
    }

    None
}

fn ensure_default_files(config_path: &Path) -> io::Result<()> {
    let config_file_path = config_path.join(CONFIG_FILE_NAME);
    if !config_file_path.exists() {
        fs::write(&config_file_path, DEFAULT_CONFIG)?;
    }

    let resources_path = config_path.join("resources");
    fs::create_dir_all(&resources_path)?;

    let text_events_path = resources_path.join("events.txt");
    if !text_events_path.exists() {
        fs::write(text_events_path, DEFAULT_TEXT_RESOURCE)?;
    }

    let csv_events_path = resources_path.join("events.csv");
    if !csv_events_path.exists() {
        fs::write(csv_events_path, DEFAULT_CSV_RESOURCE)?;
    }

    Ok(())
}