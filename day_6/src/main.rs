
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    // let mut file = File::open("test_data.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    const MAX_LENGTH: usize = 8;
    let mut histograms = [[0i32; 128]; MAX_LENGTH];

    for word in input_string.lines() {
        let bytes = word.as_bytes();
        for i in 0..bytes.len() {
            histograms[i][bytes[i] as usize] += 1;
        }
    }

    // Print most common value for each column
    let mut most_common = String::new();
    let mut least_common = String::new();
    for i in 0..MAX_LENGTH {
        // Most common
        if let Some(val_pair) = histograms[i]
            .iter()
            .enumerate()
            .map(|(chr, count)| (count, chr))
            .max() {
            let chr = (val_pair.1 as u8) as char;
            if chr >= 'a' && chr <= 'z' {
                most_common.push(chr);
            }
        }

        // Least common with non-zero count.
        if let Some(val_pair) = histograms[i]
            .iter()
            .enumerate()
            .map(|(chr, count)| (count, chr))
            .filter(|&(count, _)| *count > 0)
            .min() {
            let chr = (val_pair.1 as u8) as char;
            if chr >= 'a' && chr <= 'z' {
                least_common.push(chr);
            }
        }
    }
    println!("Part 1: '{}'", most_common);
    assert!(most_common == "zcreqgiv");

    println!("Part 2: '{}'", least_common);
    assert!(least_common == "pljvorrk");
}
