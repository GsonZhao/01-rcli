use crate::read_input;

pub fn process_sign(input: &str, key: &str, format: &str) -> Result<(), anyhow::Error> {
    let buffer = read_input(input)?;
    let key = read_input(key)?;
    let signature = match format {
        "blake3" => {
            let key_array: [u8; 32] = key[..32]
                .try_into()
                .map_err(|_| anyhow::anyhow!("Key must be 32 bytes"))?;
            blake3::keyed_hash(&key_array, &buffer).to_string()
        }
        "ed25519" => todo!(),
        _ => return Err(anyhow::anyhow!("Invalid format")),
    };
    print!("{}", signature);
    Ok(())
}

pub fn process_verify(
    input: &str,
    key: &str,
    format: &str,
    signature: &str,
) -> Result<(), anyhow::Error> {
    let buffer = read_input(input)?;
    let key = read_input(key)?;
    let signature = read_input(signature)?;
    let signature_peer = match format {
        "blake3" => {
            let key_array: [u8; 32] = key[..32]
                .try_into()
                .map_err(|_| anyhow::anyhow!("Key must be 32 bytes"))?;
            blake3::keyed_hash(&key_array, &buffer).to_string()
        }
        "ed25519" => todo!(),
        _ => return Err(anyhow::anyhow!("Invalid format")),
    };
    let signature_str =
        String::from_utf8(signature).map_err(|_| anyhow::anyhow!("Invalid signature format"))?;
    let result = signature_peer == signature_str;
    print!("{}", result);
    Ok(())
}
