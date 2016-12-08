
use std::fs::File;
use std::io::Read;

use std::collections::hash_set::HashSet;

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let num_abba = input_string.lines()
        .filter(|test_ip| valid_abba(test_ip))
        .count();

    println!("Part 1: num valid = {}", num_abba);
    assert!(num_abba == 118);

    let num_ssl = input_string.lines()
        .filter(|test_ip| valid_ssl(test_ip))
        .count();

    println!("Part 2: num valid SSL = {}", num_ssl);
    assert!(num_ssl == 260);
}

fn valid_abba(input: &str) -> bool {

    let bytes = input.as_bytes();
    const OPEN:u8 = '[' as u8;
    const CLOSE:u8 = ']' as u8;

    // Run a "convolution" kernel abba over the string, checking that
    // a==a and b==b and a!=b.
    let mut has_abba = false;

    // Also keep track of bracket [...] start stop points, to discard abba
    // patterns inside brackets
    let mut inside_brackets = false;

    // We use a 4-wide window, so stop early to avoid running off the end.
    for i in 0..(bytes.len() - 3) {
        let is_abba = bytes[i] == bytes[i + 3] && bytes[i + 1] == bytes[i + 2] &&
                      bytes[i] != bytes[i + 1];

        let has_start = bytes[i] == OPEN || bytes[i + 1] == OPEN || bytes[i + 2] == OPEN ||
            bytes[i + 3] == OPEN;

        let has_end = bytes[i] == CLOSE || bytes[i + 1] == CLOSE || bytes[i + 2] == CLOSE ||
            bytes[i + 3] == CLOSE;

        assert!(!(has_start && has_end));

        inside_brackets = (inside_brackets || has_start) && !(inside_brackets && has_end);

        if is_abba && inside_brackets {
            return false;
        }

        if is_abba {
            has_abba = has_abba || is_abba;
        }
    }
    has_abba
}

// Valid if aba outside brackets, and bab inside brackets
fn valid_ssl(input: &str) -> bool {
    let bytes = input.as_bytes();
    const OPEN:u8 = '[' as u8;
    const CLOSE:u8 = ']' as u8;

    // Run a "convolution" kernel abba over the string, checking that
    // for the strings aba (a==a and a != b) and bab (the inverse)
    // Keep track of bracket [...] start and stop points.
    let mut inside_brackets = false;

    let mut aba_set = HashSet::new();
    let mut bab_set = HashSet::new();

    // We use a 3-wide window, so stop early to avoid running off the end.
    for i in 0..(bytes.len() - 2) {
        let has_start = bytes[i] == OPEN || bytes[i + 1] == OPEN || bytes[i + 2] == OPEN;
        let has_end = bytes[i] == CLOSE || bytes[i + 1] == CLOSE || bytes[i + 2] == CLOSE;
        inside_brackets = (inside_brackets || has_start) && !(inside_brackets && has_end);

        // Check for the 'xyx' pattern
        if bytes[i] == bytes[i + 2] && bytes[i] != bytes[i + 1] {

            // Generate the inverse pattern for table query.
            let xyx = [bytes[i + 1], bytes[i], bytes[i + 1]];

            if inside_brackets {
                // As 'bab'
                if aba_set.contains(xyx.as_ref()) {
                    return true;
                }
                bab_set.insert(&bytes[i..i + 3]);
            } else {
                // As 'aba'
                if bab_set.contains(xyx.as_ref()) {
                    return true;
                }
                aba_set.insert(&bytes[i..i + 3]);
            }
        }
    }
    false
}

#[test]
fn test1() {
    assert!(valid_abba("abba[mnop]qrst") == true);
}

#[test]
fn test2() {
    assert!(valid_abba("abcd[bddb]xyyx") == false);
}

#[test]
fn test3() {
    assert!(valid_abba("aaaa[qwer]tyui") == false);
}

#[test]
fn test4() {
    assert!(valid_abba("ioxxoj[asdfgh]zxcvbn") == true);
}

#[test]
fn ssl_test1() {
    assert!(valid_ssl("aba[bab]xyz") == true);
}

#[test]
fn ssl_test2() {
    assert!(valid_ssl("xyx[xyx]xyx") == false);
}

#[test]
fn ssl_test3() {
    assert!(valid_ssl("aaa[kek]eke") == true);
}

#[test]
fn ssl_test4() {
    assert!(valid_ssl("zazbz[bzb]cdb") == true);
}