mod opts;
mod process;

pub use opts::{GenPassOptions, OutputFormat, SubCommand, Opts};
pub use process::process_csv;
pub use process::process_genpass;
