mod cli;
mod process;
mod utils;

use anyhow::Result;
pub use cli::{
    B64Format, B64SubCommand, CsvOptions, DecodeOptions, EncodeOptions, GenKeyOptions,
    GenPassOptions, HttpServerOptions, HttpSubcommand, Opts, OutputFormat, SignOptions, SubCommand,
    TextFormat, TextSubCommand, VerifyOptions,
};
pub use process::process_csv;
pub use process::process_decode;
pub use process::process_encode;
pub use process::process_genkey;
pub use process::process_genpass;
pub use process::process_http_server;
pub use process::process_sign;
pub use process::process_verify;
pub use utils::*;

use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)] // 忽略警告，在简单项目中这是可以接受的
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> Result<()>;
}
