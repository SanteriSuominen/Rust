use std::fs;
use std::path::PathBuf;

use clap::Parser;
use today::events::Category;
use today::{Config, RunOptions, parse_month_day_input, run};

const APP_NAME: &str = "today";
const CONFIG_FILE_NAME: &str = "today.toml";

#[derive(Debug, Parser)]
#[command(name = "today", version, about = "Show events with optional filters")]
struct Args {
    #[arg(short = 'a', long = "all", conflicts_with = "date")]
    all: bool,
    #[arg(short = 'd', long = "date", value_name = "MM-DD|MMDD")]
    date: Option<String>,
    #[arg(short = 'c', long = "category", value_name = "CATEGORY")]
    category: Option<String>,
    #[arg(short = 't', long = "text", value_name = "TEXT")]
    text: Option<String>,
}

fn main() {
    let normalized_args = std::env::args().map(|arg| {
        if arg == "-all" {
            "--all".to_string()
        } else {
            arg
        }
    });
    let args = Args::parse_from(normalized_args);
    let options = match build_run_options(args) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
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

            if let Err(e) = run(&config, &path, &options) {
                eprintln!("Error: {}", e);
                return;
            }
        }
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

fn build_run_options(args: Args) -> Result<RunOptions, String> {
    let month_day = match args.date {
        Some(raw_date) => Some(
            parse_month_day_input(&raw_date)
                .map_err(|err| format!("invalid --date '{}': {}", raw_date, err))?,
        ),
        None => None,
    };

    let category = args
        .category
        .as_ref()
        .map(|raw_category| Category::from_str(raw_category));

    Ok(RunOptions {
        all: args.all,
        month_day,
        category,
        text: args.text,
    })
}
