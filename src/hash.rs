use base64::Engine;
use clap::ValueEnum;
use digest::Output;
use std::fs;

#[derive(ValueEnum, Clone, Debug)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

fn generic_hasher<Hasher: digest::Digest>(bytes: &[u8]) -> Vec<u8> {
    Hasher::digest(bytes).to_vec()
}

impl HashAlgorithm {
    fn get_hasher(&self) -> Box<fn(&[u8]) -> Vec<u8>> {
        match self {
            HashAlgorithm::Md5 => Box::new(generic_hasher::<md5::Md5>),
            HashAlgorithm::Sha1 => Box::new(generic_hasher::<sha1::Sha1>),
            HashAlgorithm::Sha224 => Box::new(generic_hasher::<sha2::Sha224>),
            HashAlgorithm::Sha256 => Box::new(generic_hasher::<sha2::Sha256>),
            HashAlgorithm::Sha384 => Box::new(generic_hasher::<sha2::Sha384>),
            HashAlgorithm::Sha512 => Box::new(generic_hasher::<sha2::Sha512>),
        }
    }
}

pub struct HashImpl;
impl HashImpl {
    pub fn handle(source: String, raw: bool, algorithm: HashAlgorithm) {
        let buffer = if raw {
            source.into_bytes()
        } else {
            match fs::read(source) {
                Ok(bytes) => bytes,
                Err(err) => {
                    println!("Error: {err}");
                    return;
                }
            }
        };

        let hasher = algorithm.get_hasher();
        let result = hasher(&buffer);

        let mut hex = String::new();
        for byte in &result {
            hex += &format!("{:x}", byte);
        }
        let b64 = base64::engine::general_purpose::STANDARD.encode(&result);

        println!("HEX: {hex}");
        println!("BASE64: {b64}");
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    #[test]
    fn t() {
        HashImpl::handle("hello".to_string(), true, HashAlgorithm::Md5);
    }
}
