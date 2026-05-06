use std::fs;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use today::events::Category;
use today::{
    Config, RunOptions, add_event_to_provider, list_provider_names, parse_event_date_input,
    parse_month_day_input, run,
};

const APP_NAME: &str = "today";
const CONFIG_FILE_NAME: &str = "today.toml";

#[derive(Debug, Parser)]
#[command(name = "today", version, about = "Show events with optional filters")]
struct Cli {
    #[arg(short = 'a', long = "all", conflicts_with = "date")]
    all: bool,
    #[arg(short = 'd', long = "date", value_name = "DATE")]
    date: Option<String>,
    #[arg(short = 'e', long = "exclude", value_name = "EXCLUDE")]
    exclude: Option<String>,
    #[arg(short = 'n', long = "no-birthday")]
    no_birthday: bool,
    #[arg(short = 'c', long = "category", value_name = "CATEGORY")]
    category: Option<String>,
    #[arg(short = 't', long = "text", value_name = "TEXT")]
    text: Option<String>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List all event providers
    Providers,
    /// Add an event to an event provider
    Add(AddCommand),
}

#[derive(Debug, Args)]
struct AddCommand {
    #[arg(short = 'p', long = "provider", value_name = "PROVIDER")]
    provider: String,
    #[arg(short = 'd', long = "date", value_name = "DATE")]
    date: String,
    #[arg(short = 'e', long = "description", value_name = "DESCRIPTION")]
    description: String,
    #[arg(short = 'c', long = "category", value_name = "CATEGORY")]
    category: String,
}

fn main() {
    let normalized_args = std::env::args().map(|arg| {
        if arg == "-all" {
            "--all".to_string()
        } else {
            arg
        }
    });
    let cli = Cli::parse_from(normalized_args);

    let (config, config_path) = match load_config(APP_NAME, CONFIG_FILE_NAME) {
        Ok(values) => values,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match &cli.command {
        Some(Command::Providers) => {
            for provider_name in list_provider_names(&config) {
                println!("{}", provider_name);
            }
            return;
        }
        Some(Command::Add(add_command)) => {
            let date = match parse_event_date_input(&add_command.date) {
                Ok(date) => date,
                Err(err) => {
                    eprintln!("Error: invalid --date '{}': {}", add_command.date, err);
                    return;
                }
            };
            let category = Category::from_str(&add_command.category);
            if let Err(err) = add_event_to_provider(
                &config,
                &config_path,
                &add_command.provider,
                date,
                &add_command.description,
                &category,
            ) {
                eprintln!("Error: {}", err);
                return;
            }
            println!("Added event to provider '{}'.", add_command.provider);
            return;
        }
        None => {}
    }

    let options = match build_run_options(&cli) {
        Ok(options) => options,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    if let Err(e) = run(&config, &config_path, &options) {
        eprintln!("Error: {}", e);
        return;
    }
}

fn load_config(app_name: &str, config_file_name: &str) -> Result<(Config, PathBuf), String> {
    let config_path = get_config_path(app_name)
        .ok_or_else(|| "Unable to configure the application".to_string())?;
    let toml_path = config_path.join(config_file_name);
    let config_str = fs::read_to_string(&toml_path).map_err(|err| {
        format!(
            "Unable to read configuration file '{}': {}",
            toml_path.display(),
            err
        )
    })?;
    let config = toml::from_str(&config_str).map_err(|err| {
        format!(
            "Unable to parse configuration file '{}': {}",
            toml_path.display(),
            err
        )
    })?;
    Ok((config, config_path))
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

fn build_run_options(cli: &Cli) -> Result<RunOptions, String> {
    let month_day = match cli.date.as_ref() {
        Some(raw_date) => Some(
            parse_month_day_input(raw_date)
                .map_err(|err| format!("invalid --date '{}': {}", raw_date, err))?,
        ),
        None => None,
    };

    let category = cli
        .category
        .as_ref()
        .map(|raw_category| Category::from_str(raw_category));
    let excluded_categories = cli
        .exclude
        .as_deref()
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(Category::from_str)
                .collect::<Vec<Category>>()
        })
        .unwrap_or_default();

    Ok(RunOptions {
        all: cli.all,
        no_birthday: cli.no_birthday,
        month_day,
        category,
        text: cli.text.clone(),
        excluded_categories,
    })
}
