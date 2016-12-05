
use std::u32;
use std::fmt::Write;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

// It takes two hex digits to make a byte, so check
// the first 2.5 bytes for zero values
fn first_five_hex_zero(input: &[u8; 16]) -> bool {
    let sum = (input[0] as u32) + (input[1] as u32) + (input[2] as u32 >> 4);
    sum == 0
}

fn get_sixth_hex_char(input: &[u8; 16]) -> char {
    let mut buff = String::new();
    let _ = write!(buff,"{:x}", input[2]);
    buff.as_bytes()[0] as char
}

fn main() {

    // Puzzle input
    let door_id = "reyedfim";

    let mut hasher = Md5::new();
    let mut password = "".to_owned();
    let target_password_length = 8;

    // Static buffer to hold the result
    let mut hash_output = [0u8; 16];

    // Loop through door_id + [0..max] to find hashes that
    // start with five zeros
    for i in 0..u32::max_value() {
        let mut test_value = String::new();
        let _ = write!(test_value,"{}{}", door_id, i);
        hasher.reset();
        hasher.input(test_value.as_bytes());
        hasher.result(&mut hash_output);

        if first_five_hex_zero(&hash_output) {
            let next_char = get_sixth_hex_char(&hash_output);
            //let next_char = hash_output[0] as char;
            password.push(next_char);
        }

        if password.len() >= target_password_length {
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

    let mut buff = [0;16];
    hasher.result(&mut buff);
    let test_char1 = get_sixth_hex_char(&buff);
    println!("{}", test_char1);
    //assert!(test_char1 == '');

    let test_value2 = "abc5017308";
    hasher.reset();
    hasher.input(test_value2.as_bytes());
    let hash_result2 = hasher.result_str();
    println!("{}", hash_result2);
}
