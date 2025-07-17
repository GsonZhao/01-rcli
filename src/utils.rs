use std::io::Read;

pub fn read_input(input: &str) -> Result<Vec<u8>, anyhow::Error> {
    if input == "-" {
        let mut buffer = Vec::new();
        std::io::stdin().read_to_end(&mut buffer)?;
        Ok(buffer)
    } else {
        Ok(std::fs::read(input)?)
    }
}
