use std::{fmt::Display, path::PathBuf, str::FromStr};

use crate::cli::verify_path;

#[derive(clap::Subcommand)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a text", long_about = None)]
    Sign(SignOptions),
    #[command(name = "verify", about = "Verify a text", long_about = None)]
    Verify(VerifyOptions),
    #[command(name = "genkey", about = "Generate a key", long_about = None)]
    GenKey(GenKeyOptions),
}

#[derive(clap::Parser, Debug)]
pub struct SignOptions {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, value_parser = parse_text_format, default_value = "blake3")]
    pub format: TextFormat,
}

#[derive(clap::Parser, Debug)]
pub struct VerifyOptions {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, value_parser = parse_text_format, default_value = "blake3")]
    pub format: TextFormat,
    #[arg(short, long)]
    pub signature: String,
}

#[derive(clap::Parser, Debug)]
pub struct GenKeyOptions {
    #[arg(short, long, value_parser = parse_text_format, default_value = "blake3")]
    pub format: TextFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(clap::Parser, Clone, Copy, Debug)]
pub enum TextFormat {
    Blake3,
    Ed25519,
}

fn parse_text_format(s: &str) -> Result<TextFormat, anyhow::Error> {
    s.parse()
}

impl Display for TextFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<TextFormat> for &str {
    fn from(format: TextFormat) -> Self {
        match format {
            TextFormat::Blake3 => "blake3",
            TextFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextFormat::Blake3),
            "ed25519" => Ok(TextFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text format")),
        }
    }
}
