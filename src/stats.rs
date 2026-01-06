/* src/stats.rs */

pub struct ProcessStats {
    modified: Vec<String>,
    needs_update: Vec<String>,
    skipped: Vec<(String, String)>,
    warnings: Vec<(String, String)>,
    errors: Vec<(String, String)>,
}

impl ProcessStats {
    pub fn new() -> Self {
        ProcessStats {
            modified: Vec::new(),
            needs_update: Vec::new(),
            skipped: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_modified(&mut self, path: String) {
        self.modified.push(path);
    }

    pub fn add_needs_update(&mut self, path: String) {
        self.needs_update.push(path);
    }

    pub fn add_skipped(&mut self, path: String, reason: String) {
        self.skipped.push((path, reason));
    }

    pub fn add_warning(&mut self, path: String, reason: String) {
        self.warnings.push((path, reason));
    }

    pub fn add_error(&mut self, path: String, reason: String) {
        self.errors.push((path, reason));
    }

    pub fn print_results(&self, check_mode: bool) {
        let total_modified = self.modified.len();
        let total_needs_update = self.needs_update.len();
        let total_skipped = self.skipped.len();
        let _total_warnings = self.warnings.len();
        let total_errors = self.errors.len();

        // Only print summary if something happened
        if check_mode {
            if total_needs_update > 0 {
                println!();
                if total_needs_update == 1 {
                    println!("1 file needs update");
                } else {
                    println!("{} files need updates", total_needs_update);
                }
            }
        } else {
            if total_modified > 0 || total_skipped > 0 || total_errors > 0 {
                println!();

                let mut parts = Vec::new();

                if total_modified > 0 {
                    if total_modified == 1 {
                        parts.push("1 file updated".to_string());
                    } else {
                        parts.push(format!("{} files updated", total_modified));
                    }
                }

                if total_skipped > 0 {
                    if total_skipped == 1 {
                        parts.push("1 skipped".to_string());
                    } else {
                        parts.push(format!("{} skipped", total_skipped));
                    }
                }

                if total_errors > 0 {
                    if total_errors == 1 {
                        parts.push("1 error".to_string());
                    } else {
                        parts.push(format!("{} errors", total_errors));
                    }
                }

                println!("{}", parts.join(", "));
            }
        }
    }

    pub fn get_exit_code(&self, check_mode: bool) -> i32 {
        if !self.errors.is_empty() {
            return 1;
        }

        if check_mode && !self.needs_update.is_empty() {
            return 1;
        }

        0
    }
}
