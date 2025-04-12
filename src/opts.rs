use std::path::Path;

#[derive(clap::Parser)]
#[command(name = "rcli", about = "A rust command line interface", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(clap::Subcommand)]
pub enum SubCommand {
    Csv(CsvOptions),
}

#[derive(clap::Parser)]
#[command(name = "csv", about = "A csv command", long_about = None)]
pub struct CsvOptions {
    #[arg(short, long, required = true, help = "The input csv file", value_parser = validate_file)]
    pub input: String,
    #[arg(
        short,
        long,
        default_value = "output.json",
        help = "The output csv file"
    )]
    pub output: String,
    #[arg(long, default_value = "true", help = "The header")]
    pub header: bool,
    #[arg(long, default_value = ",", help = "The delimiter")]
    pub delimiter: String,
}

fn validate_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
