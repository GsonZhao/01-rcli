use std::{fmt::Display, str::FromStr};

use super::verify_input_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(clap::Parser)]
pub struct CsvOptions {
    #[arg(short, long, required = true, help = "The input csv file", value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, help = "The output csv file")]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json", help = "The output format")]
    pub format: OutputFormat,
    #[arg(long, default_value = "true", help = "The header")]
    pub header: bool,
    #[arg(long, default_value = ",", help = "The delimiter")]
    pub delimiter: String,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for OutputFormat {
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
