use base64::{engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE, Engine as _};
use std::io::{BufReader, Read};

use crate::B64Format;

pub fn process_encode(input: &str, format: &B64Format) -> anyhow::Result<()> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    let mut reader = BufReader::new(reader);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let encoded = match format {
        B64Format::Standard => STANDARD.encode(&buffer),
        B64Format::UrlSafe => URL_SAFE.encode(&buffer),
    };
    println!("{encoded}");

    Ok(())
}

pub fn process_decode(input: &str, format: &B64Format) -> anyhow::Result<()> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    let mut reader = BufReader::new(reader);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let decoded = match format {
        B64Format::Standard => STANDARD.decode(&buffer),
        B64Format::UrlSafe => URL_SAFE.decode(&buffer),
    };
    println!("{}", String::from_utf8(decoded?)?);
    Ok(())
}
