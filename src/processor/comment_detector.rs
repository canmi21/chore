/* src/processor/comment_detector.rs */

/// Check if a line looks like a path comment
pub fn looks_like_path_comment(line: &str) -> bool {
    let trimmed = line.trim();

    // Check for common comment patterns with paths
    if trimmed.starts_with("//") || trimmed.starts_with("#") {
        // Single-line comments
        let content = if trimmed.starts_with("//") {
            trimmed.trim_start_matches("//").trim()
        } else {
            trimmed.trim_start_matches("#").trim()
        };

        // Check if it looks like a file path (contains / or ends with extension)
        return content.contains('/')
            || content.contains('\\')
            || content.ends_with(".rs")
            || content.ends_with(".js")
            || content.ends_with(".py")
            || content.ends_with(".ts")
            || content.ends_with(".cpp")
            || content.ends_with(".c")
            || content.ends_with(".go")
            || content.ends_with(".java");
    }

    if trimmed.starts_with("/*") && trimmed.ends_with("*/") {
        // Multi-line comment on single line
        let content = trimmed
            .trim_start_matches("/*")
            .trim_end_matches("*/")
            .trim();

        return content.contains('/')
            || content.contains('\\')
            || content.ends_with(".rs")
            || content.ends_with(".js")
            || content.ends_with(".cpp")
            || content.ends_with(".c");
    }

    if trimmed.starts_with("<!--") && trimmed.ends_with("-->") {
        // HTML comment
        let content = trimmed
            .trim_start_matches("<!--")
            .trim_end_matches("-->")
            .trim();

        return content.contains('/') || content.ends_with(".html");
    }

    false
}

/// Find where actual content starts (skip existing path comments and leading blank lines)
pub fn find_content_start(lines: &[&str]) -> usize {
    let mut start_idx = 0;

    // Skip first line if it looks like a path comment
    if !lines.is_empty() && looks_like_path_comment(lines[0]) {
        start_idx = 1;
    }

    // Skip leading blank lines
    while start_idx < lines.len() && lines[start_idx].trim().is_empty() {
        start_idx += 1;
    }

    start_idx
}

/// Find where actual content starts after shebang
pub fn find_content_start_after_shebang(lines: &[&str]) -> usize {
    let mut start_idx = 1; // Start after shebang

    // Skip second line if it looks like a path comment
    if lines.len() > 1 && looks_like_path_comment(lines[1]) {
        start_idx = 2;
    }

    // Skip leading blank lines
    while start_idx < lines.len() && lines[start_idx].trim().is_empty() {
        start_idx += 1;
    }

    start_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_path_comment() {
        assert!(looks_like_path_comment("// src/main.rs"));
        assert!(looks_like_path_comment("/* src/main.rs */"));
        assert!(looks_like_path_comment("# scripts/deploy.py"));
        assert!(looks_like_path_comment("<!-- index.html -->"));

        assert!(!looks_like_path_comment("// This is a normal comment"));
        assert!(!looks_like_path_comment("/* TODO: fix this */"));
    }

    #[test]
    fn test_find_content_start() {
        let lines = vec!["// src/main.rs", "", "fn main() {"];
        assert_eq!(find_content_start(&lines), 2);

        let lines = vec!["fn main() {"];
        assert_eq!(find_content_start(&lines), 0);

        let lines = vec!["", "", "fn main() {"];
        assert_eq!(find_content_start(&lines), 2);
    }
}
