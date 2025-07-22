use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

use crate::{
    cli::verify_path, get_content, get_reader, process_genkey, process_sign, process_verify,
    CmdExecutor,
};
use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[derive(clap::Subcommand)]
#[enum_dispatch(CmdExecutor)]
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

// impl CmdExecutor for TextSubCommand {
//     async fn execute(self) -> Result<()> {
//         match self {
//             TextSubCommand::Sign(opts) => opts.execute().await,
//             TextSubCommand::Verify(opts) => opts.execute().await,
//             TextSubCommand::GenKey(opts) => opts.execute().await,
//         }
//     }
// }

impl CmdExecutor for SignOptions {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_sign(&mut reader, &key, self.format)?;
        let encoded = URL_SAFE_NO_PAD.encode(&sig);
        print!("{encoded}");
        Ok(())
    }
}

impl CmdExecutor for VerifyOptions {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let signature = get_content(&self.signature)?;
        let sig = URL_SAFE_NO_PAD.decode(signature)?;
        let result = process_verify(&mut reader, &key, &sig, self.format)?;
        if result {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}

impl CmdExecutor for GenKeyOptions {
    async fn execute(self) -> Result<()> {
        let key = process_genkey(self.format)?;
        for (name, value) in key {
            fs::write(self.output.join(name), value)?;
        }
        Ok(())
    }
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
