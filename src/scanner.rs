/* src/scanner.rs */

use crate::config::Config;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Scanner {
    config: Config,
    project_root: PathBuf,
}

#[derive(Debug)]
pub enum SkipReason {
    ExcludedByDir(String),
    ExcludedByPattern(String),
    NoMatchingFormat,
    NotUtf8,
    SymbolicLink,
}

impl Scanner {
    pub fn new(config: Config, project_root: PathBuf) -> Self {
        Scanner {
            config,
            project_root,
        }
    }

    /// Collect all files that should be processed
    pub fn collect_files(&self, target_path: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();

        if target_path.is_file() {
            // Single file mode
            files.push(target_path.to_path_buf());
        } else if target_path.is_dir() {
            // Directory mode: walk recursively
            for entry in WalkDir::new(target_path).follow_links(false) {
                if let Ok(entry) = entry {
                    if entry.file_type().is_file() {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }

        files
    }

    /// Check if a file should be processed or skipped
    pub fn should_process(&self, file_path: &Path) -> Result<(), SkipReason> {
        // Check if it's a symbolic link
        if file_path.is_symlink() {
            return Err(SkipReason::SymbolicLink);
        }

        // First check if file extension has a matching format
        // Only files we intend to process should be checked for exclusion
        if !self.has_matching_format(file_path) {
            return Err(SkipReason::NoMatchingFormat);
        }

        // Then check exclude rules
        // Now we only check exclusion for files that would otherwise be processed
        if let Some(reason) = self.check_exclude_rules(file_path) {
            return Err(reason);
        }

        // Check if file is valid UTF-8
        if let Ok(content) = std::fs::read(file_path) {
            if std::str::from_utf8(&content).is_err() {
                return Err(SkipReason::NotUtf8);
            }
        }

        Ok(())
    }

    /// Check if file matches any exclude rules
    fn check_exclude_rules(&self, file_path: &Path) -> Option<SkipReason> {
        let exclude = &self.config.path_comment.exclude;

        // Get relative path from project root
        let rel_path = if let Ok(rel) = file_path.strip_prefix(&self.project_root) {
            rel
        } else {
            file_path
        };

        // Check excluded directories
        for excluded_dir in &exclude.dirs {
            if self.path_contains_dir(rel_path, excluded_dir) {
                return Some(SkipReason::ExcludedByDir(excluded_dir.clone()));
            }
        }

        // Check excluded patterns
        for pattern in &exclude.patterns {
            if self.matches_pattern(file_path, pattern) {
                return Some(SkipReason::ExcludedByPattern(pattern.clone()));
            }
        }

        None
    }

    /// Check if path contains a specific directory component
    fn path_contains_dir(&self, path: &Path, dir_name: &str) -> bool {
        for component in path.components() {
            if let Some(name) = component.as_os_str().to_str() {
                if name == dir_name {
                    return true;
                }
            }
        }
        false
    }

    /// Check if file matches a glob pattern
    fn matches_pattern(&self, file_path: &Path, pattern: &str) -> bool {
        if let Some(file_name) = file_path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if let Ok(glob_pattern) = glob::Pattern::new(pattern) {
                    return glob_pattern.matches(file_name_str);
                }
            }
        }
        false
    }

    /// Check if file extension has a matching format in config
    fn has_matching_format(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension() {
            let ext_with_dot = format!(".{}", ext.to_string_lossy());
            self.config.path_comment.formats.contains_key(&ext_with_dot)
        } else {
            false
        }
    }

    /// Get the comment format for a file
    pub fn get_format(&self, file_path: &Path) -> Option<&str> {
        if let Some(ext) = file_path.extension() {
            let ext_with_dot = format!(".{}", ext.to_string_lossy());
            self.config
                .path_comment
                .formats
                .get(&ext_with_dot)
                .map(|s| s.as_str())
        } else {
            None
        }
    }
}
