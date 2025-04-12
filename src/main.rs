use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    match opts.command {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
            Ok(())
        }
    }
}
