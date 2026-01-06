/* src/config/mod.rs */

mod defaults;

pub use defaults::default_formats;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathCommentConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    #[serde(default)]
    pub formats: HashMap<String, String>,

    #[serde(default)]
    pub exclude: ExcludeConfig,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExcludeConfig {
    #[serde(default)]
    pub dirs: Vec<String>,

    #[serde(default)]
    pub patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub path_comment: PathCommentConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path_comment: PathCommentConfig {
                enabled: true,
                formats: default_formats(),
                exclude: ExcludeConfig::default(),
            },
        }
    }
}

impl Config {
    /// Load config from file, or return default config
    pub fn load(config_path: Option<PathBuf>) -> Result<Self, String> {
        if let Some(path) = config_path {
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;

            let config: Config = toml::from_str(&content)
                .map_err(|e| format!("Failed to parse config file: {}", e))?;

            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Find config file in current or parent directories
    pub fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
        let mut current = start_dir.to_path_buf();

        loop {
            // Check for chore.toml first
            let chore_toml = current.join("chore.toml");
            if chore_toml.exists() {
                return Some(chore_toml);
            }

            // Then check for .chore.toml
            let dot_chore_toml = current.join(".chore.toml");
            if dot_chore_toml.exists() {
                return Some(dot_chore_toml);
            }

            // Move to parent directory
            if !current.pop() {
                break;
            }
        }

        None
    }

    /// Generate default config file content for --init
    pub fn generate_init_config(project_dir: &Path, max_depth: usize) -> String {
        // Scan project to find which file types exist
        let found_extensions = Self::scan_for_extensions(project_dir, max_depth);

        // Get default formats
        let all_formats = default_formats();

        // Filter to only include formats for extensions found in project
        let mut active_formats: Vec<(String, String)> = all_formats
            .into_iter()
            .filter(|(ext, _)| found_extensions.contains(ext))
            .collect();

        // Sort for consistent output
        active_formats.sort_by(|a, b| a.0.cmp(&b.0));

        // Generate TOML content
        let mut content =
            String::from("[path_comment]\nenabled = true\n\n[path_comment.formats]\n");

        for (ext, format) in active_formats {
            content.push_str(&format!("\"{}\" = \"{}\"\n", ext, format));
        }

        content.push_str("\n[path_comment.exclude]\ndirs = []\npatterns = []\n");

        content
    }

    /// Scan project directory to find which file extensions exist (up to max_depth levels)
    fn scan_for_extensions(project_dir: &Path, max_depth: usize) -> Vec<String> {
        use walkdir::WalkDir;

        let mut extensions = Vec::new();
        let default_formats = default_formats();

        for entry in WalkDir::new(project_dir)
            .max_depth(max_depth)
            .follow_links(false)
        {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        let ext_str = format!(".{}", ext.to_string_lossy());
                        if default_formats.contains_key(&ext_str) && !extensions.contains(&ext_str)
                        {
                            extensions.push(ext_str);
                        }
                    }
                }
            }
        }

        extensions
    }
}
