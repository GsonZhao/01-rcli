mod cli;
mod process;

pub use cli::{B64Format, B64SubCommand, Opts, OutputFormat, SubCommand};
pub use process::process_csv;
pub use process::process_decode;
pub use process::process_encode;
pub use process::process_genpass;
