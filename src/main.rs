use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, B64SubCommand, Opts, SubCommand,
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
            );
            println!("{password}");
            Ok(())
        }
        SubCommand::B64(command) => {
            match command {
                B64SubCommand::Encode(opts) => {
                    process_encode(&opts.input, &opts.format)?;
                }
                B64SubCommand::Decode(opts) => {
                    process_decode(&opts.input, &opts.format)?;
                }
            }
            Ok(())
        }
    }
}
