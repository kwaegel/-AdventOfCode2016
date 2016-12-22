
use std::fmt::Write;
use std::collections::VecDeque;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn to_hex_string(bytes: &[u8; 16]) -> Result<String, std::fmt::Error> {

    let mut s = String::new();
    for &byte in bytes.iter() {
        write!(&mut s, "{:02x}", byte)?;
    }
    Ok(s)
}

fn has_triplet(hash: &str) -> Option<char> {
    let bytes = hash.as_bytes();
    for i in 1..bytes.len()-1 {
        if bytes[i-1] == bytes[i] && bytes[i] == bytes[i+1] {
            return Some(bytes[i] as char);
        }
    }
    None
}

fn has_quintuplet(hash: &str) -> Option<char> {
    let bytes = hash.as_bytes();
    for i in 0..bytes.len()-5 {
        if bytes[i] == bytes[i+1]
            && bytes[i] == bytes[i+2]
            && bytes[i] == bytes[i+3]
            && bytes[i] == bytes[i+4] {
            return Some(bytes[i] as char);
        }
    }
    None
}

fn main() {

    let salt = "ngcjuoqr";
    //let salt = "abc";

    // Bounded queue, storing (id, char) pairs.
    let mut keys = Vec::with_capacity(64);
    let mut history: VecDeque<(u32, char)> = VecDeque::with_capacity(5000);

    // Static buffer to hold the hash output
    let mut hasher = Md5::new();
    let mut hash_output = [0u8; 16];

    // Loop through door_id + [0..max] to find hashes that
    // start with five zeros
    for i in 0..u32::max_value() {
        let mut test_value = String::new();
        let _ = write!(test_value,"{}{}", salt, i);
        hasher.reset();
        hasher.input(test_value.as_bytes());
        hasher.result(&mut hash_output);

        // Check hash for character triplets.
        let hash = to_hex_string(&hash_output).expect("Failed to write hash");

        // Prune the history to only the last 1000 keys.
        let oldest = i as i32 - 1000_i32 + 1;
        while let Some(&(idx, _)) = history.front() {
            if (idx as i32) < oldest {
                history.pop_front();
            } else {
                break;
            }
        }

        // if quintuplet, check the last 1000 keys.
        if let Some(chr) = has_quintuplet(&hash) {

            for &(prev_idx, prev_chr) in history.iter() {
                if prev_chr == chr {
                    //println!("Found key {} at {}", prev_idx, i);
                    keys.push(prev_idx);
                }
            }

            history.retain(|&x| x.1 != chr);

            history.push_back((i, chr));
        }
        if let Some(chr) = has_triplet(&hash) {
            history.push_back((i, chr));
        }

        if keys.len() >= 64 {
            break;
        }
    }

    // Part 1: check the index of the 64-th key
    keys.sort();
    keys.dedup();

    println!("{} keys: {:?}", keys.len(), keys);
    println!("Part 1: index of 64-th key: {}", keys[63]);
    assert!(keys.len() >= 64);
    assert!(keys[63] == 18626);
}
