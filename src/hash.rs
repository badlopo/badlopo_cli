use clap::ValueEnum;
use image::EncodableLayout;
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

fn generic_hasher<Hasher: digest::Digest>(bytes: &[u8]) -> String {
    let result = Hasher::digest(bytes);
    let mut hex = String::new();
    for byte in result {
        hex += &format!("{:x}", byte);
    }
    hex
}

impl HashAlgorithm {
    fn hasher(&self) -> Box<fn(&[u8]) -> String> {
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

        let hasher = algorithm.hasher();
        let result = hasher(&buffer);
        println!("{result}");
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    #[test]
    fn t() {
        HashImpl::handle("hello".to_string(), Some(true), HashAlgorithm::Md5);
    }
}
