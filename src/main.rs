/* src/main.rs */

mod cli;
mod file_processor;
mod init;
mod stats;

use chore_cli::{Config, Processor, Scanner};
use clap::Parser;
use cli::Cli;
use file_processor::process_file;
use init::handle_init;
use stats::ProcessStats;
use std::process;

fn main() {
    let cli = Cli::parse();

    // Handle --init flag
    if cli.init {
        handle_init();
        return;
    }

    // Ensure at least one path is provided
    if cli.paths.is_empty() {
        eprintln!("Error: No paths specified. Use 'chore <PATH>' or 'chore --help' for usage.");
        process::exit(1);
    }

    // Get current working directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error: Failed to get current directory: {}", e);
            process::exit(1);
        }
    };

    // Find project root
    let project_root = chore_cli::path_resolver::find_project_root(&current_dir);

    // Load configuration
    let config_file = Config::find_config_file(&current_dir);
    let config = match Config::load(config_file) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Check if path comment feature is enabled
    if !config.path_comment.enabled {
        eprintln!("Warning: path_comment feature is disabled in config");
        return;
    }

    // Process all target paths
    let mut stats = ProcessStats::new();

    let scanner = Scanner::new(config.clone(), project_root.clone());
    let processor = Processor::new(project_root.clone());

    for target_path in &cli.paths {
        let full_path = if target_path.is_absolute() {
            target_path.clone()
        } else {
            current_dir.join(target_path)
        };

        if !full_path.exists() {
            stats.add_error(
                target_path.display().to_string(),
                "Path does not exist".to_string(),
            );
            continue;
        }

        let files = scanner.collect_files(&full_path);

        for file in files {
            process_file(
                &scanner,
                &processor,
                &file,
                &project_root,
                cli.check,
                &mut stats,
            );
        }
    }

    // Print results
    stats.print_results(cli.check);

    // Set exit code
    let exit_code = stats.get_exit_code(cli.check);
    process::exit(exit_code);
}
