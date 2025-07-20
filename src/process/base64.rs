use std::io::Read;

use base64::{
    engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD, Engine as _,
};

use crate::B64Format;

pub fn process_encode(reader: &mut dyn Read, format: &B64Format) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        B64Format::Standard => STANDARD.encode(&buf),
        B64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    print!("{encoded}");

    Ok(())
}

pub fn process_decode(reader: &mut dyn Read, format: &B64Format) -> anyhow::Result<()> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim_end();
    let decoded = match format {
        B64Format::Standard => STANDARD.decode(buf),
        B64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf),
    };
    println!("{}", String::from_utf8(decoded?)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::get_reader;

    use super::*;

    #[test]
    fn test_encode_standard() -> anyhow::Result<()> {
        let input = "Cargo.toml";
        let format = B64Format::Standard;
        let mut reader = get_reader(input)?;
        process_encode(&mut reader, &format)?;
        Ok(())
    }

    #[test]
    fn test_decode_url_safe() -> anyhow::Result<()> {
        let input = "fixtures/b64.txt";
        let format = B64Format::UrlSafe;
        let mut reader = get_reader(input)?;
        process_decode(&mut reader, &format)?;
        Ok(())
    }
}
