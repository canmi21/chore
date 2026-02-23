/* src/file_processor.rs */

use crate::stats::ProcessStats;
use chore_cli::{Processor, Scanner};
use std::path::Path;

pub fn process_file(
    scanner: &Scanner,
    processor: &Processor,
    file_path: &Path,
    project_root: &Path,
    check_only: bool,
    stats: &mut ProcessStats,
) {
    // Get relative path for display
    let display_path = file_path
        .strip_prefix(project_root)
        .unwrap_or(file_path)
        .to_string_lossy()
        .replace('\\', "/");

    // Check if file should be processed
    match scanner.should_process(file_path) {
        Ok(()) => {
            // Get format template
            let format = match scanner.get_format(file_path) {
                Some(f) => f,
                None => {
                    // This shouldn't happen as should_process checks for format
                    return;
                }
            };

            // Process the file
            match processor.process_file(file_path, format, check_only) {
                chore_cli::processor::ProcessResult::Modified => {
                    if check_only {
                        println!("Needs update: {}", display_path);
                        stats.add_needs_update(display_path);
                    } else {
                        println!("Modified: {}", display_path);
                        stats.add_modified(display_path);
                    }
                }
                chore_cli::processor::ProcessResult::AlreadyCorrect => {
                    // Silent when already correct
                }
                chore_cli::processor::ProcessResult::Skipped(reason) => {
                    stats.add_skipped(display_path, reason);
                }
                chore_cli::processor::ProcessResult::Error(reason) => {
                    println!("Error: {} ({})", display_path, reason);
                    stats.add_error(display_path, reason);
                }
            }
        }
        Err(reason) => {
            use chore_cli::scanner::SkipReason;
            match reason {
                SkipReason::ExcludedByDir(dir) => {
                    stats.add_skipped(display_path, format!("excluded by dir: {}", dir));
                }
                SkipReason::ExcludedByPattern(pattern) => {
                    stats.add_skipped(display_path, format!("excluded by pattern: {}", pattern));
                }
                SkipReason::NotUtf8 => {
                    println!("Warning: {} (Not valid UTF-8)", display_path);
                    stats.add_warning(display_path, "Not valid UTF-8".to_string());
                }
                SkipReason::SymbolicLink => {
                    // Silently skip symbolic links
                }
                SkipReason::NoMatchingFormat => {
                    // Silently skip files without matching format
                }
            }
        }
    }
}
