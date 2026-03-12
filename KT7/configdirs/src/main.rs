use std::path::PathBuf;
use std::fs;
use dirs;

fn main() {
    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => println!("Config path: '{}'", path.display()),
        None => println!("No config directory found!")
    }
}

// Gets the configuration directory path for the application
// named in the `app_name` argument.
// If the directory does not exist, tries to create it.
// Returns an optional `PathBuf` containing the directory path,
// or None if the directory can't be created.
fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        println!("Config directory: '{}'", config_dir.display());

        // Check if our config directory exists
        let config_path = config_dir.join(app_name);
        print!("App config directory: '{}'", config_path.display());

        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            } else {
                print!(" - created");
            }
        } else {
            print!(" - exists");
        }
        println!();

        return Some(config_path);
    }

    None
}