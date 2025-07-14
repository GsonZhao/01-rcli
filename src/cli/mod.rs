mod csv;
mod genpass;

use csv::CsvOptions;
use genpass::GenPassOptions;

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
}
