/* src/cli.rs */

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "chore")]
#[command(version = "0.1.0")]
#[command(about = "A tool to add path comments to source files", long_about = None)]
pub struct Cli {
    /// Target paths to process (files or directories)
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Check mode: only report files that need updates without modifying them
    #[arg(long)]
    pub check: bool,

    /// Initialize configuration file
    #[arg(long)]
    pub init: bool,
}
