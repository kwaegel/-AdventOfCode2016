
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

    // Part 1: process by rows
    let mut valid = 0;
    for line in input_string.lines() {
        let values = line.split(' ')
            .filter(|ref val| val.len() > 0) // Filter contiguous separators.
            .map(|ref val| val.parse().unwrap())
            .collect::<Vec<u32>>();
        if valid_triangle(values[0], values[1], values[2]) {
            valid += 1;
        }
    }
    println!("Day 1: valid triangles = {}", valid);
    assert!(valid == 1050);

    // Part 2: process by column in groups of three.
    let all_values = input_string.split(|c| c == ' ' || c == '\n')
        .filter(|ref val| val.len() > 0) // Filter contiguous separators
        .map(|ref val| val.parse().unwrap()) // Convert to integer
        .collect::<Vec<u32>>();

    // Process values with a stride of 9 (for each block of three triangles)
    let mut valid_by_column = 0;
    for block_stride in 0..all_values.len()/9 {
        for stride in 0..3 {
            let a = all_values[block_stride*9 + stride + 0];
            let b = all_values[block_stride*9 + stride + 3];
            let c = all_values[block_stride*9 + stride + 6];
            if valid_triangle(a,b,c) {
                valid_by_column += 1;
            }
        }
    }
    println!("Day 1: valid triangles by column = {}", valid_by_column);

}
