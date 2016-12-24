
use std::fmt::Write;
use std::collections::VecDeque;
use std::collections::HashMap;

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


fn compute_keys(salt: &str, extra_md5_iters: u32) -> Vec<u32> {

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
        let mut hash = to_hex_string(&hash_output).expect("Failed to write hash");

        // Implement key stretching
        for _ in 0..extra_md5_iters {
            hasher.reset();
            hasher.input(hash.as_bytes());
            hasher.result(&mut hash_output);
            hash = to_hex_string(&hash_output).expect("Failed to write hash");
        }

        // Prune the history to only the last 1000 keys.
        while let Some(&(idx, _)) = history.front() {

            if idx + 1000 < i {
                history.pop_front();
            } else {
                break;
            }
        }

        // if quintuplet, check the last 1000 keys.
        if let Some(chr) = has_quintuplet(&hash) {

            for &(prev_idx, prev_chr) in history.iter() {
                if prev_chr == chr {
                    let diff = i - prev_idx;
                    println!("Found key at {} using quintuplet {} at {} (diff {})",
                             prev_idx, chr, i, diff);
                    keys.push(prev_idx);
                }
            }

            history.retain(|&x| x.1 != chr);

            history.push_back((i, chr));
            println!("-----")
        }
        else if let Some(chr) = has_triplet(&hash) {
            history.push_back((i, chr));
        }

        if keys.len() >= 64 {
            break;
        }
    }

    keys
}


fn compute_keys_2(salt: &str, extra_md5_iters: u32) -> Vec<u32> {

    let mut triplets = Vec::new();
    let mut quintuplets: HashMap<char, Vec<u32>> = HashMap::new();

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
        let mut hash = to_hex_string(&hash_output).expect("Failed to write hash");

        // Implement key stretching
        for _ in 0..extra_md5_iters {
            hasher.reset();
            hasher.input(hash.as_bytes());
            hasher.result(&mut hash_output);
            hash = to_hex_string(&hash_output).expect("Failed to write hash");
        }

        // if quintuplet, check the last 1000 keys.
        if let Some(chr) = has_quintuplet(&hash) {
            let entry = quintuplets.entry(chr).or_insert(Vec::new());
            entry.push(i);

        }
        if let Some(chr) = has_triplet(&hash) {
            triplets.push((i, chr));
        }

        if i > 20000 {
            break;
        }
    }

    let mut keys = Vec::new();

    for tri in &triplets {
        if let Some(quint) = quintuplets.get(&tri.1) {
            for quint_index in quint {
                if *quint_index > tri.0 && tri.0 + 1000 <= *quint_index {
                    keys.push(tri.0);
                    break;
                }
            }
        }
    }

    println!("triplets = {}, quintuplets = {}", triplets.len(), quintuplets.len());

    //triplets
    //keys.dedup();
    keys
}


fn main() {

    // Part 1
    {
        let keys = compute_keys("ngcjuoqr", 0);
        println!("{} keys: {:?}", keys.len(), keys);
        println!("Part 1: index of 64-th key: {}", keys[63]);
        assert!(keys.len() >= 64);
        assert!(keys[63] == 18626);
    }

    // Part 2
    {
        let keys = compute_keys("ngcjuoqr", 2016);
        println!("{} keys: {:?}", keys.len(), &keys);
        println!("Part 2: index of 64-th key: {}", keys[63]);
        assert!(keys.len() >= 64);
        assert!(keys[63] != 1521);
        assert!(keys[63] < 20199);
        assert!(keys[63] < 20219);
    }
}

#[test]
fn iterated_keys() {
    let val = "abc0";

    let mut hasher = Md5::new();
    let mut hash_output = [0u8; 16];

    hasher.reset();
    hasher.input(val.as_bytes());
    hasher.result(&mut hash_output);
    let mut hash = to_hex_string(&hash_output).expect("Failed to write hash");

    // Implement key stretching
    for _ in 0..2016 {
        hasher.reset();
        hasher.input(hash.as_bytes());
        hasher.result(&mut hash_output);
        hash = to_hex_string(&hash_output).expect("Failed to write hash");
    }

    println!("hash = {}", hash);
    assert!(hash == "a107ff634856bb300138cac6568c0f24");
}

#[test]
fn part1_sample() {
    let keys = compute_keys("abc", 0);
    assert!(keys.len() >= 64);
    assert!(keys[63] == 22728);
}

#[test]
fn part2_sample() {
    let keys = compute_keys("abc", 2016);
    println!("{} keys: {:?}", keys.len(), &keys[0..63]);
    assert!(keys.len() >= 64);
    assert!(keys[63] == 22551);
}