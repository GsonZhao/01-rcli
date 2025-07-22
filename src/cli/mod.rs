mod b64;
mod csv;
mod genpass;
mod http_server;
mod text;

use std::path::{Path, PathBuf};

use enum_dispatch::enum_dispatch;

pub use csv::CsvOptions;

pub use b64::{B64Format, B64SubCommand, DecodeOptions, EncodeOptions};
pub use http_server::HttpSubcommand;
pub use text::{TextFormat, TextSubCommand};

pub use csv::OutputFormat;
pub use genpass::GenPassOptions;
pub use http_server::HttpServerOptions;
pub use text::{GenKeyOptions, SignOptions, VerifyOptions};

#[derive(clap::Parser)]
#[command(name = "rcli", about = "A rust command line interface", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(clap::Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "A csv command", long_about = None)]
    Csv(CsvOptions),
    #[command(name = "genpass", about = "A genpass command", long_about = None)]
    GenPass(GenPassOptions),
    #[command(subcommand, name = "base64", about = "A base64 command", long_about = None)]
    B64(B64SubCommand),
    #[command(subcommand,name = "text", about = "A text command", long_about = None)]
    Text(TextSubCommand),
    #[command(subcommand, name = "http", about = "A http file server", long_about = None)]
    Http(HttpSubcommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(p.to_path_buf())
    } else {
        Err("Path is not a directory")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        let filename = "Cargo.toml";
        assert!(verify_file(filename).is_ok());
    }
}
