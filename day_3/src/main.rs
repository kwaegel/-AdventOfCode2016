
use std::fs::File;
use std::io::Read;

fn valid_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c
    && a + c > b
    && b + c > a
}

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let mut valid = 0;
    for line in input_string.lines() {
        let values = line.split(" ")
            .filter(|ref val| val.len() > 0) // Filter contiguous separators.
            .map(|ref val| val.parse().unwrap())
            .collect::<Vec<u32>>();
        if valid_triangle(values[0], values[1], values[2]) {
            valid += 1;
        }
    }
    println!("Day 1: valid triangles = {}", valid);
    assert!(valid == 1050);
}
