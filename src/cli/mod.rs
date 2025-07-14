mod b64;
mod csv;
mod genpass;

use std::path::Path;

use csv::CsvOptions;
use genpass::GenPassOptions;

pub use b64::{B64Format, B64SubCommand};

pub use csv::OutputFormat;

#[derive(clap::Parser)]
#[command(name = "rcli", about = "A rust command line interface", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(clap::Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "A csv command", long_about = None)]
    Csv(CsvOptions),
    #[command(name = "genpass", about = "A genpass command", long_about = None)]
    GenPass(GenPassOptions),
    #[command(subcommand, name = "base64", about = "A base64 command", long_about = None)]
    B64(B64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
