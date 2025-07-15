use base64::{
    engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD, Engine as _,
};
use std::io::{BufReader, Read};

use crate::B64Format;

pub fn process_encode(input: &str, format: &B64Format) -> anyhow::Result<()> {
    let buffer = read_input(input)?;
    let encoded = match format {
        B64Format::Standard => STANDARD.encode(&buffer),
        B64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buffer),
    };
    print!("{encoded}");

    Ok(())
}

pub fn process_decode(input: &str, format: &B64Format) -> anyhow::Result<()> {
    let buffer = read_input(input)?;
    let buffer_str = String::from_utf8(buffer.clone())?;
    let buffer_str = buffer_str.trim_end();
    let decoded = match format {
        B64Format::Standard => STANDARD.decode(buffer_str.as_bytes()),
        B64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer_str.as_bytes()),
    };
    println!("{}", String::from_utf8(decoded?)?);
    Ok(())
}

fn read_input(input: &str) -> Result<Vec<u8>, anyhow::Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    let mut reader = BufReader::new(reader);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_standard() {
        let input = "Cargo.toml";
        let format = B64Format::Standard;
        assert!(process_encode(input, &format).is_ok());
    }

    #[test]
    fn test_decode_url_safe() {
        let input = "fixtures/b64.txt";
        let format = B64Format::UrlSafe;
        process_decode(input, &format).unwrap();
    }
}
