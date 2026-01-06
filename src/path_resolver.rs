/* src/path_resolver.rs */

use std::path::{Path, PathBuf};

/// Find the project root directory based on various heuristics
pub fn find_project_root(start_dir: &Path) -> PathBuf {
    // Priority 1: Look for chore.toml or .chore.toml
    if let Some(config_dir) = find_config_directory(start_dir) {
        return config_dir;
    }

    // Priority 2: Look for .git directory
    if let Some(git_root) = find_git_root(start_dir) {
        return git_root;
    }

    // Priority 3: Look for Cargo.toml
    if let Some(cargo_root) = find_cargo_root(start_dir) {
        return cargo_root;
    }

    // Priority 4: Use current working directory
    start_dir.to_path_buf()
}

/// Find directory containing chore.toml or .chore.toml
fn find_config_directory(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();

    loop {
        if current.join("chore.toml").exists() || current.join(".chore.toml").exists() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

/// Find Git repository root
fn find_git_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();

    loop {
        if current.join(".git").exists() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

/// Find Cargo project root
fn find_cargo_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();

    loop {
        if current.join("Cargo.toml").exists() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_project_root() {
        // Test with current directory (should find the Cargo.toml)
        let current_dir = std::env::current_dir().unwrap();
        let root = find_project_root(&current_dir);

        // Should find Cargo.toml in project root
        assert!(root.join("Cargo.toml").exists());
    }
}
