
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    const LENGTH: usize = 8;
    let mut histograms = [[0i32; 128]; LENGTH];

    for word in input_string.lines() {
        let bytes = word.as_bytes();
        for i in 0..LENGTH {
            histograms[i][bytes[i] as usize] += 1;
        }
    }

    // Print most common value for each column
    let mut word = String::new();
    for i in 0..LENGTH {
        if let Some(val_pair) = histograms[i].iter().enumerate().map(|(x, y)| (y, x)).max() {
            let chr = (val_pair.1 as u8) as char;
            word.push(chr);
            //println!("{:?} = {}->{}", val_pair, val_pair.1, chr);
        }
    }
    println!("Part 1: {}", word);
    assert!(word == "zcreqgiv")

}
