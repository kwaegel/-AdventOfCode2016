
//extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

use utils::to_hex_string;

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

    pub fn hash(&mut self, input: &str, iters: u32) -> String {
        assert!(iters > 0, "Invalid number of iterations");
        // Implement key stretching
        let mut hash = String::from(input);
        for _ in 0..iters {
            self.hasher.reset();
            self.hasher.input(hash.as_bytes());
            self.hasher.result(&mut self.output_buff);
            hash = to_hex_string(&self.output_buff).expect("Failed to write hash");
        }
        hash
    }
}

#[cfg(test)]
mod test {

    use hasher::Hasher;

    #[test]
    fn simple_hash() {
        let mut hasher = Hasher::new();
        let hash = hasher.hash("abc0", 1);
        println!("hash = {}", hash);
        assert_eq!(hash, "577571be4de9dcce85a041ba0410f29f");
    }


    #[test]
    fn iterated_hash() {
        let mut hasher = Hasher::new();
        let hash = hasher.hash("abc0", 2017);
        println!("hash = {}", hash);
        assert_eq!(hash, "a107ff634856bb300138cac6568c0f24");
    }

    #[test]
    fn day1_simple() {
        let mut hasher = Hasher::new();
        let mut hash = hasher.hash("abc39", 1);
        println!("hash('abc39') = {}", hash);

        hash = hasher.hash("abc816", 1);
        println!("hash('abc816') = {}", hash);
        //assert_eq!(hash, "0034e0923cc38887a57bd7b1d4f953df");
    }

    #[test]
    fn day1_simple2() {
        let mut hasher = Hasher::new();
        let mut hash = hasher.hash("abc92", 1);
        println!("hash('abc92') = {}", hash);

        hash = hasher.hash("abc200", 1);
        println!("hash('abc200') = {}", hash);
        //assert_eq!(hash, "0034e0923cc38887a57bd7b1d4f953df");

        hash = hasher.hash("abc110", 1);
        println!("hash('abc110') = {}", hash);
        //assert_eq!(hash, "0034e0923cc38887a57bd7b1d4f953df");
    }
}
