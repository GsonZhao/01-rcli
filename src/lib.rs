mod cli;
mod process;

pub use cli::{Opts, OutputFormat, SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
