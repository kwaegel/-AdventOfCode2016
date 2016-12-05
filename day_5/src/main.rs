
use std::u32;
use std::fmt::Write;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

// Returns the sixth character if the first five are zero
fn sixth_char_after_zeros(hex_str: &str) -> Option<char> {

    for i in 0..5 {
        if hex_str.as_bytes()[i] != '0' as u8 {
            return None;
        }
    }
    Some(hex_str.as_bytes()[5] as char)
}

fn main() {

    // Puzzle input
    let door_id = "reyedfim";

    let mut hasher = Md5::new();
    let mut password = "".to_owned();
    let target_password_length = 8;

    // Loop through door_id + [0..max] to find hashes that
    // start with five zeros
    for i in 0..u32::max_value() {
        let mut test_value = String::new();
        let _ = write!(test_value,"{}{}", door_id, i);
        hasher.reset();
        hasher.input(test_value.as_bytes());
        
        let hash_result = hasher.result_str();
        if let Some(next_char) = sixth_char_after_zeros(&hash_result) {
            println!("{}", hash_result);
            password.push(next_char);
        }

        if password.len() == target_password_length {
            break;
        }
    }
    if password.len() < target_password_length {
        println!("Could not find password. Latest attempt: {}", password)
    } else {
        println!("Part1: password = '{}'", password);
    }
    assert!(password == "f97c354d");
}

#[test]
fn test1() {
    let test_value = "abc3231929";
    let mut hasher = Md5::new();
    hasher.input(test_value.as_bytes());
    let hash_result = hasher.result_str();
    println!("{}", hash_result);

    let test_value2 = "abc5017308";
    hasher.reset();
    hasher.input(test_value2.as_bytes());
    let hash_result2 = hasher.result_str();
    println!("{}", hash_result2);
}