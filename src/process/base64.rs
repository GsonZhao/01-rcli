use base64::{engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE, Engine as _};

use crate::{read_input, B64Format};

pub fn process_encode(input: &str, format: &B64Format) -> anyhow::Result<()> {
    let buffer = read_input(input)?;

    let encoded = match format {
        B64Format::Standard => STANDARD.encode(buffer),
        B64Format::UrlSafe => URL_SAFE.encode(buffer),
    };
    println!("{encoded}");

    Ok(())
}

pub fn process_decode(input: &str, format: &B64Format) -> anyhow::Result<()> {
    let buffer = read_input(input)?;
    let decoded = match format {
        B64Format::Standard => STANDARD.decode(buffer),
        B64Format::UrlSafe => URL_SAFE.decode(buffer),
    };
    println!("{}", String::from_utf8(decoded?)?);
    Ok(())
}
