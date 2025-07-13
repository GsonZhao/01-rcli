use std::{fmt::Display, path::Path, str::FromStr};

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

#[derive(Debug,Clone,Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(clap::Parser)]
pub struct CsvOptions {
    #[arg(short, long, required = true, help = "The input csv file", value_parser = validate_file)]
    pub input: String,
    #[arg(
        short,
        long,
        help = "The output csv file"
    )]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json", help = "The output format")]
    pub format: OutputFormat,
    #[arg(long, default_value = "true", help = "The header")]
    pub header: bool,
    #[arg(long, default_value = ",", help = "The delimiter")]
    pub delimiter: String,
}

#[derive(clap::Parser, Debug)]
pub struct GenPassOptions {
    #[arg(short,long, default_value_t = 16, help = "The length of the password")]
    pub length: u8,
    #[arg(long, default_value_t = true, help = "Include uppercase letters")]
    pub uppercase: bool,
    #[arg(long, default_value_t = true, help = "Include lowercase letters")]
    pub lowercase: bool,
    #[arg(long, default_value_t = true, help = "Include numbers")]
    pub numbers: bool,
    #[arg(long, default_value_t = true, help = "Include symbols")]
    pub symbols: bool,
}


fn validate_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for OutputFormat{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid output format: {}", s)),
        }
    }
}

impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}