use std::path::PathBuf;

#[derive(clap::Subcommand, Debug)]
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
