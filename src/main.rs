use clap::Parser;
use rcli::{process_csv, process_genpass, SubCommand, Opts};

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
            let password = process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.numbers, opts.symbols);
            println!("{}", password);
            Ok(()) 
        }
    }
}
