use anyhow::Result;
use rand::seq::SliceRandom;

const UPPERCASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();

    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("char won't be empty in this context"),
        );
    }

    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("char won't be empty in this context"),
        );
    }

    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(
            *NUMBERS
                .choose(&mut rng)
                .expect("char won't be empty in this context"),
        );
    }

    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(
            *SYMBOLS
                .choose(&mut rng)
                .expect("char won't be empty in this context"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("char won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    Ok(String::from_utf8_lossy(&password).to_string())
}
