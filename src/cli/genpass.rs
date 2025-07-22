use crate::{process_genpass, CmdExecutor};

#[derive(clap::Parser, Debug)]
pub struct GenPassOptions {
    #[arg(short, long, default_value_t = 16, help = "The length of the password")]
    pub length: u8,
    #[arg(long, default_value_t = true, help = "Include uppercase letters")]
    pub uppercase: bool,
    #[arg(long, default_value_t = true, help = "Include lowercase letters")]
    pub lowercase: bool,
    #[arg(long, default_value_t = true, help = "Include numbers")]
    pub numbers: bool,
    #[arg(long, default_value_t = true, help = "Include symbols")]
    pub symbols: bool,
}
use anyhow::Result;

impl CmdExecutor for GenPassOptions {
    async fn execute(self) -> Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbols,
        )?;
        let score = zxcvbn::zxcvbn(&password, &[]).score();
        print!("{password}");
        eprintln!(" 密码强度: {}", score);
        Ok(())
    }
}
