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
