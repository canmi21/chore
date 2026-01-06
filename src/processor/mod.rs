/* src/processor/mod.rs */

mod comment_detector;

use comment_detector::{find_content_start, find_content_start_after_shebang};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessResult {
    Modified,
    AlreadyCorrect,
    Skipped(String),
    Error(String),
}

pub struct Processor {
    project_root: PathBuf,
}

impl Processor {
    pub fn new(project_root: PathBuf) -> Self {
        Processor { project_root }
    }

    /// Process a single file: add or update path comment
    pub fn process_file(
        &self,
        file_path: &Path,
        format_template: &str,
        check_only: bool,
    ) -> ProcessResult {
        // Read file content
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => return ProcessResult::Error(format!("Failed to read file: {}", e)),
        };

        // Get relative path from project root
        let rel_path = match file_path.strip_prefix(&self.project_root) {
            Ok(rel) => rel,
            Err(_) => file_path,
        };

        let rel_path_str = rel_path.to_string_lossy().replace('\\', "/");

        // Generate expected path comment
        let expected_comment = format_template.replace("$path$file", &rel_path_str);

        // Check if file needs processing
        let needs_update = self.check_needs_update(&content, &expected_comment);

        if !needs_update {
            return ProcessResult::AlreadyCorrect;
        }

        if check_only {
            return ProcessResult::Modified;
        }

        // Process the file
        match self.add_path_comment(&content, &expected_comment) {
            Ok(new_content) => {
                if let Err(e) = fs::write(file_path, new_content) {
                    ProcessResult::Error(format!("Failed to write file: {}", e))
                } else {
                    ProcessResult::Modified
                }
            }
            Err(e) => ProcessResult::Error(e),
        }
    }

    /// Check if file needs updating
    fn check_needs_update(&self, content: &str, expected_comment: &str) -> bool {
        let lines: Vec<&str> = content.lines().collect();

        if lines.is_empty() {
            return true;
        }

        // Check for shebang
        let has_shebang = lines[0].starts_with("#!");

        // Determine which line should contain the path comment
        let comment_line_idx = if has_shebang { 1 } else { 0 };

        // Check if file has enough lines
        if lines.len() <= comment_line_idx {
            return true;
        }

        // Check if the comment line matches expected
        let actual_line = lines[comment_line_idx].trim();
        let expected_line = expected_comment.trim();

        actual_line != expected_line
    }

    /// Add path comment to file content
    fn add_path_comment(&self, content: &str, expected_comment: &str) -> Result<String, String> {
        let lines: Vec<&str> = content.lines().collect();

        // Check for shebang
        let has_shebang = if !lines.is_empty() && lines[0].starts_with("#!") {
            true
        } else {
            false
        };

        let mut new_lines = Vec::new();

        if has_shebang {
            // Keep shebang as first line
            new_lines.push(lines[0].to_string());

            // Add path comment as second line
            new_lines.push(expected_comment.to_string());

            // Add blank line
            new_lines.push(String::new());

            // Determine where to start copying original content
            let start_idx = find_content_start_after_shebang(&lines);

            // Add remaining content
            for line in lines.iter().skip(start_idx) {
                new_lines.push(line.to_string());
            }
        } else {
            // Add path comment as first line
            new_lines.push(expected_comment.to_string());

            // Add blank line
            new_lines.push(String::new());

            // Determine where to start copying original content
            let start_idx = find_content_start(&lines);

            // Add remaining content
            for line in lines.iter().skip(start_idx) {
                new_lines.push(line.to_string());
            }
        }

        // Join with LF line endings
        Ok(new_lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_add_path_comment_simple() {
        let processor = Processor::new(PathBuf::from("/project"));

        let content = "fn main() {\n    println!(\"Hello\");\n}";
        let result = processor.add_path_comment(content, "/* src/main.rs */");

        assert!(result.is_ok());
        let new_content = result.unwrap();

        assert!(new_content.starts_with("/* src/main.rs */\n\nfn main()"));
    }

    #[test]
    fn test_add_path_comment_with_shebang() {
        let processor = Processor::new(PathBuf::from("/project"));

        let content = "#!/usr/bin/env python3\nimport sys";
        let result = processor.add_path_comment(content, "# scripts/deploy.py");

        assert!(result.is_ok());
        let new_content = result.unwrap();

        assert!(
            new_content.starts_with("#!/usr/bin/env python3\n# scripts/deploy.py\n\nimport sys")
        );
    }
}
