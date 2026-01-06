/* src/lib.rs */

pub mod config;
pub mod path_resolver;
pub mod processor;
pub mod scanner;

pub use config::Config;
pub use processor::{ProcessResult, Processor};
pub use scanner::Scanner;
