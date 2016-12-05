
use std::str;
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

fn get_seventh_hex_char(input: &[u8; 16]) -> char {
    let mut buff = String::new();
    let _ = write!(buff,"{:x}", input[3] >> 4);
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
            password.push(next_char);
        }

        if password.len() >= target_password_length {
            break;
        }
    }
    println!("Part 1: password = '{}'", password);
    assert!(password == "f97c354d");

    // Part 2
    let mut password_part2_bytes = [0u8; 8]; //"        ".as_bytes();
    let mut set_characters = 0;
    // Loop through door_id + [0..max] to find hashes that
    // start with five zeros
    for i in 0..u32::max_value() {
        let mut test_value = String::new();
        let _ = write!(test_value,"{}{}", door_id, i);
        hasher.reset();
        hasher.input(test_value.as_bytes());
        hasher.result(&mut hash_output);

        if first_five_hex_zero(&hash_output) {
            let next_pos = get_sixth_hex_char(&hash_output);
            let next_char = get_seventh_hex_char(&hash_output);

            if let Some(index) = next_pos.to_digit(10) {
                if index < 8 {
                    if password_part2_bytes[index as usize] == 0 {
                        password_part2_bytes[index as usize] = next_char as u8;
                        set_characters += 1;
                    }
                }
            }
        }

        if set_characters == target_password_length {
            break;
        }
    }

    let password_part2 = str::from_utf8(&password_part2_bytes).unwrap();
    println!("Part 2: password = '{}'", password_part2);
    assert!(password_part2 == "863dde27");
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
    let position = get_sixth_hex_char(&buff);
    let character = get_seventh_hex_char(&buff);
    println!("pos: {}", position);
    println!("chr: {}", character);
}
