use anyhow::Result;

use clap::Parser;
use rcli::CmdExecutor;
use rcli::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();
    opts.command.execute().await
}
