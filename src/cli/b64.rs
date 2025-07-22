use std::str::FromStr;

use crate::{get_reader, process_decode, process_encode, CmdExecutor};

use super::verify_file;
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Parser, Debug)]
pub struct B64Options {
    #[command(subcommand)]
    pub command: B64SubCommand,
}

#[enum_dispatch(CmdExecutor)]
#[derive(Debug, Parser)]
pub enum B64SubCommand {
    #[command(name = "encode", about = "Encode a file", long_about = None)]
    Encode(EncodeOptions),
    #[command(name = "decode", about = "Decode a file", long_about = None)]
    Decode(DecodeOptions),
}

// impl CmdExecutor for B64SubCommand {
//     async fn execute(self) -> Result<()> {
//         match self {
//             B64SubCommand::Encode(opts) => opts.execute().await,
//             B64SubCommand::Decode(opts) => opts.execute().await,
//         }
//     }
// }

impl CmdExecutor for EncodeOptions {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        process_encode(&mut reader, &self.format)?;
        Ok(())
    }
}

impl CmdExecutor for DecodeOptions {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        process_decode(&mut reader, &self.format)?;
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct EncodeOptions {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "Standard")]
    pub format: B64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOptions {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
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
