use std::str::FromStr;

use super::verify_input_file;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct B64Options {
    #[command(subcommand)]
    pub command: B64SubCommand,
}

#[derive(Debug, Parser)]
pub enum B64SubCommand {
    #[command(name = "encode", about = "Encode a file", long_about = None)]
    Encode(EncodeOptions),
    #[command(name = "decode", about = "Decode a file", long_about = None)]
    Decode(DecodeOptions),
}

#[derive(Debug, Parser)]
pub struct EncodeOptions {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "Standard")]
    pub format: B64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOptions {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "Standard")]
    pub format: B64Format,
}

#[derive(Debug, Clone)]
pub enum B64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<B64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for B64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(B64Format::Standard),
            "urlsafe" => Ok(B64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format: {}", s)),
        }
    }
}
