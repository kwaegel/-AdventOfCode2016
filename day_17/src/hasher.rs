
use std;
use std::fmt::Write;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn to_hex_string(bytes: &[u8; 16]) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    for &byte in bytes.iter() {
        write!(&mut s, "{:02x}", byte)?;
    }
    Ok(s)
}


pub fn hash(input: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.hash(input)
}


pub struct Hasher {
    hasher: Md5,
    output_buff: [u8; 16],
}

impl Hasher {
    pub fn new() -> Hasher {
        Hasher {
            hasher: Md5::new(),
            output_buff: [0u8; 16],
        }
    }

    pub fn hash(&mut self, input: &str) -> String {
        self.hasher.reset();
        self.hasher.input(input.as_bytes());
        self.hasher.result(&mut self.output_buff);
        to_hex_string(&self.output_buff).expect("Failed to write hash")
    }
}