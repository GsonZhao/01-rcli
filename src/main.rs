use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genkey,
    process_genpass, process_sign, process_verify, B64SubCommand, Opts, SubCommand, TextSubCommand,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, &output, opts.format)?;
            Ok(())
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
            let score = zxcvbn::zxcvbn(&password, &[]).score();
            println!("{password} ({score})");
            Ok(())
        }
        SubCommand::B64(command) => {
            match command {
                B64SubCommand::Encode(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    process_encode(&mut reader, &opts.format)?;
                }
                B64SubCommand::Decode(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    process_decode(&mut reader, &opts.format)?;
                }
            }
            Ok(())
        }
        SubCommand::Text(command) => {
            match command {
                TextSubCommand::Sign(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    let key = get_content(&opts.key)?;
                    let sig = process_sign(&mut reader, &key, opts.format)?;
                    let encoded = URL_SAFE_NO_PAD.encode(&sig);
                    println!("{encoded}");
                }
                TextSubCommand::Verify(opts) => {
                    let mut reader = get_reader(&opts.input)?;
                    let key = get_content(&opts.key)?;
                    let signature = URL_SAFE_NO_PAD.decode(&opts.signature)?;
                    let verified = process_verify(&mut reader, &key, &signature, opts.format)?;
                    if verified {
                        println!("✓ Signature verified")
                    } else {
                        println!("⚠ Signature not verified");
                    }
                }
                TextSubCommand::GenKey(opts) => {
                    let key = process_genkey(opts.format)?;
                    for (name, value) in key {
                        fs::write(opts.output.join(name), value)?;
                    }
                }
            }
            Ok(())
        }
    }
}
