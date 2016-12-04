
#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// Returns (name, sector_id, stored checksum)
fn unpack(input: &str) -> (&str, i32, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r##"([-a-z]+)-([:digit:]+)\[([:alpha:]+)\]"##).unwrap();
    }

    let cap = RE.captures(input).unwrap();
    let name = cap.at(1).unwrap();
    let sector_id: i32 = cap.at(2).unwrap().parse().unwrap();
    let checksum = cap.at(3).unwrap();

    (name, sector_id, checksum)
}

fn compute_checksum(input: &str) -> String {
    let histogram = input.chars()
        .filter(|&c| c >= 'a' && c <= 'z')
        .fold(HashMap::new(), |mut histogram, c| {
            *histogram.entry(c).or_insert(0) += 1;
            histogram
        });

    // Collect the histogram as a vec of (letter, count) pairs, then sort.
    let mut pairs = histogram.iter().collect::<Vec<_>>();
    pairs.sort_by(|a,b| {
        match b.1.cmp(a.1) { // First by descending count {3,2,1}
            Ordering::Equal => a.0.cmp(b.0), // Then by lexicographic order {a,b,c}
            o => o,
        }
    });

    pairs.iter()
        .take(5)
        .map(|&(&letter, _)| letter)
        .fold(String::new(), |mut checksum, c| {checksum.push(c); checksum})
}

// Returns the sector ID if room is valid, else None.
// Format: "a-b-c-d-e-f-g-h-987[abcde]"
fn verify_checksum(input: &str) -> Option<i32> {
    let (name, sector_id, stored_checksum) = unpack(input);
    let checksum = compute_checksum(name);
    if checksum == stored_checksum {Some(sector_id)} else {None}
}


fn shift_char(c: char, shift: i32) -> char {
    let code = (((c as i32 - 'a' as i32 + shift) % 26) + 'a' as i32) as u32;
    std::char::from_u32(code).unwrap()
}

fn decrypt(input: &str) -> (String, i32) {
    let (name, shift, _) = unpack(input);
    let name = name.chars()
        .map(|c| if c == '-' {' '} else {shift_char(c, shift)})
        .fold(String::new(), |mut string, c| {string.push(c); string});
    (name, shift)
}

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    // Accumulate the sector ids of all correct rooms.
    let sum = input_string.lines()
        .map(verify_checksum)
        .filter_map(|x| x)
        .fold(0, |acc, id| acc + id);

    println!("Part 1: sum of valid sector IDs = {:?}", sum);
    assert!(sum == 185371);

    // Part 2: decrypt names
    let decrypted_names = input_string.lines()
        .filter(|val| verify_checksum(val).is_some())
        .map(decrypt)
        .filter(|&(ref string, _)| string.contains("north"))
        .collect::<Vec<_>>();
    for val in &decrypted_names {
        println!("{:?}", val);
    }
    assert!(decrypted_names[0].1 == 984);
}

#[test]
fn test1() {
    assert!(compute_checksum("aaaaa-bbb-z-y-x-123[abxyz]") == "abxyz");
    assert!(compute_checksum("a-b-c-d-e-f-g-h-987[abcde]") == "abcde");
    assert!(compute_checksum("not-a-real-room-404[oarel]") == "oarel");
    assert!(compute_checksum("totally-real-room-200[decoy]") != "decoy");

    assert!(verify_checksum("aaaaa-bbb-z-y-x-123[abxyz]") == Some(123));
    assert!(verify_checksum("a-b-c-d-e-f-g-h-987[abcde]") == Some(987));
    assert!(verify_checksum("not-a-real-room-404[oarel]") == Some(404));
    assert!(verify_checksum("totally-real-room-200[decoy]") == None);
}

#[test]
fn decryption_test() {
    assert!(decrypt("qzmt-zixmtkozy-ivhz-343[blank]") == "very encrypted name");
}