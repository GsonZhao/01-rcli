use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{collections::HashMap, io::Read};

use crate::{process_genpass, TextFormat};
use anyhow::Result;

trait TextSign {
    fn sign(&self, data: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, input: &mut dyn Read, signature: &[u8]) -> Result<bool>;
}

trait KeyLoader {
    fn load(key: impl AsRef<[u8]>) -> Result<Self>
    where
        Self: Sized;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, data: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, input: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        Ok(hash.as_bytes() == signature)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, data: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer)?;
        let signature = self.key.sign(&buffer);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, input: &mut dyn Read, signature: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;
        let signature = Signature::from_bytes(signature.try_into()?);
        Ok(self.key.verify(&buffer, &signature).is_ok())
    }
}

pub fn process_sign(reader: &mut dyn Read, key: &[u8], format: TextFormat) -> Result<Vec<u8>> {
    let signed: Box<dyn TextSign> = match format {
        TextFormat::Blake3 => Box::new(Blake3::load(key)?),
        TextFormat::Ed25519 => Box::new(Ed25519Signer::load(key)?),
    };
    signed.sign(reader)
}

pub fn process_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextFormat,
) -> Result<bool, anyhow::Error> {
    let verified: Box<dyn TextVerify> = match format {
        TextFormat::Blake3 => Box::new(Blake3::load(key)?),
        TextFormat::Ed25519 => Box::new(Ed25519Verifier::load(key)?),
    };
    verified.verify(reader, sig)
}

pub fn process_genkey(format: TextFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextFormat::Blake3 => Blake3::generate(),
        TextFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key: [u8; 32] = key.as_ref()[..32].try_into()?;
        Ok(Self::new(key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        Ok(HashMap::from([("blake3.txt", key.as_bytes().to_vec())]))
    }
}

impl Ed25519Signer {
    fn new(key: [u8; 32]) -> Self {
        let key = SigningKey::from_bytes(&key);
        Self { key }
    }

    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key: [u8; 32] = key.as_ref()[..32].try_into()?;
        Ok(Self::new(key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key();
        Ok(HashMap::from([
            ("ed25519.sk", sk.to_bytes().to_vec()),
            ("ed25519.pk", pk.to_bytes().to_vec()),
        ]))
    }
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key: [u8; 32] = key.as_ref()[..32].try_into()?;
        let key = VerifyingKey::from_bytes(&key)?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Blake3 {
    fn load(key: impl AsRef<[u8]>) -> Result<Self> {
        Self::try_new(key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(key: impl AsRef<[u8]>) -> Result<Self> {
        Self::try_new(key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(key: impl AsRef<[u8]>) -> Result<Self> {
        Self::try_new(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");

    #[test]
    fn test_process_text_sign() -> Result<()> {
        let mut reader = b"Hello, world!".as_slice();
        let mut reader1 = b"Hello, world!".as_slice();
        let signature = process_sign(&mut reader, KEY, TextFormat::Blake3)?;
        let verified = process_verify(&mut reader1, KEY, &signature, TextFormat::Blake3)?;
        assert!(verified);
        Ok(())
    }
}
