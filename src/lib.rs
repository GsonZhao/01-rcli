mod opts;
mod process;

pub use opts::{GenPassOptions, Opts, OutputFormat, SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
