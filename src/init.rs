/* src/init.rs */

use chore_cli::Config;
use std::process;

pub fn handle_init() {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error: Failed to get current directory: {}", e);
            process::exit(1);
        }
    };

    // Check if config already exists in current directory
    let chore_toml = current_dir.join("chore.toml");
    let dot_chore_toml = current_dir.join(".chore.toml");

    if chore_toml.exists() {
        eprintln!("Error: chore.toml already exists in current directory");
        process::exit(1);
    }

    if dot_chore_toml.exists() {
        eprintln!("Error: .chore.toml already exists in current directory");
        process::exit(1);
    }

    // Generate config content by scanning project (max 5 levels)
    let config_content = Config::generate_init_config(&current_dir, 5);

    // Write to chore.toml
    if let Err(e) = std::fs::write(&chore_toml, config_content) {
        eprintln!("Error: Failed to write chore.toml: {}", e);
        process::exit(1);
    }

    println!("Created chore.toml in current directory");
}
