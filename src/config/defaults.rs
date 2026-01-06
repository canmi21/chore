/* src/config/defaults.rs */

use std::collections::HashMap;

/// Default comment formats for different file types
pub fn default_formats() -> HashMap<String, String> {
    let mut formats = HashMap::new();

    // Languages with /* */ style comments
    formats.insert(".rs".to_string(), "/* $path$file */".to_string());
    formats.insert(".c".to_string(), "/* $path$file */".to_string());
    formats.insert(".cpp".to_string(), "/* $path$file */".to_string());
    formats.insert(".h".to_string(), "/* $path$file */".to_string());
    formats.insert(".css".to_string(), "/* $path$file */".to_string());
    formats.insert(".js".to_string(), "/* $path$file */".to_string());
    formats.insert(".ts".to_string(), "/* $path$file */".to_string());
    formats.insert(".jsx".to_string(), "/* $path$file */".to_string());
    formats.insert(".tsx".to_string(), "/* $path$file */".to_string());
    formats.insert(".go".to_string(), "/* $path$file */".to_string());

    // Languages with // style comments
    formats.insert(".java".to_string(), "// $path$file".to_string());
    formats.insert(".kt".to_string(), "// $path$file".to_string());
    formats.insert(".swift".to_string(), "// $path$file".to_string());

    // Languages with # style comments
    formats.insert(".py".to_string(), "# $path$file".to_string());
    formats.insert(".sh".to_string(), "# $path$file".to_string());

    // HTML special case
    formats.insert(".html".to_string(), "<!-- $path$file -->".to_string());

    formats
}
