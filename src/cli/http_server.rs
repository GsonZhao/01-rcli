use crate::{process_http_server, CmdExecutor};
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(clap::Subcommand, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubcommand {
    #[command(name = "serve", about = "Serve a directory over HTTP")]
    Serve(HttpServerOptions),
}

#[derive(clap::Parser, Debug)]
pub struct HttpServerOptions {
    #[arg(long, default_value = ".", help = "The path to the directory to serve")]
    pub path: PathBuf,
    #[arg(long, default_value = "8080", help = "The port to listen on")]
    pub port: u16,
}

// impl CmdExecutor for HttpSubcommand {
//     async fn execute(self) -> Result<()> {
//         match self {
//             HttpSubcommand::Serve(opts) => opts.execute().await,
//         }
//     }
// }

impl CmdExecutor for HttpServerOptions {
    async fn execute(self) -> Result<()> {
        process_http_server(self.path, self.port).await?;
        Ok(())
    }
}
