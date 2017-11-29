
use std::fmt::Write;

fn is_even(val: usize) -> bool {
    val % 2 == 0
}

fn generate_dragon(initial: &Vec<u8>, target_size: usize) -> Vec<u8> {
    let mut a = initial.clone();
    while a.len() < target_size {
        let mut b = a.clone();
        b.reverse();
        for val in b.iter_mut() {
            *val = if *val == 1u8 { 0u8 } else { 1u8 };
        }
        a.push(0);
        a.append(&mut b);
    }
    a.truncate(target_size);
    a
}

fn as_vec(input: &str) -> Vec<u8> {
    input.chars().map(|c| c.to_digit(2).unwrap() as u8).collect()
}

fn as_str(input: &[u8]) -> String {
    let mut out = String::new();
    for val in input.iter() {
        write!(&mut out, "{}", val).unwrap();
    }
    out
}

fn checksum(input: &Vec<u8>) -> String {
    let mut result = input.clone();
    //
    while is_even(result.len()) {
        let mut next_result = Vec::new();

        // Iterate over pairs
        for chunk in result.chunks(2) {
            let i: u8 = chunk[0]; // chunk[0].unwrap();
            let j: u8 = chunk[1]; // chunk[1].unwrap();
            let next_bit = if i == j {1} else {0};
            //println!("[{}, {}] -> {}", i, j, next_bit);
            next_result.push(next_bit);
        }
        result = next_result;
    }


    let mut out_str = String::new();
    for bit in result {
        write!(&mut out_str, "{}", bit).unwrap();
    }
    out_str
}

fn main() {

    // Example data
    let example_curve = generate_dragon(&vec!{1,0,0,0,0}, 20);
    let example_checksum = checksum(&example_curve);
    println!("Example curve: {}", as_str(&example_curve));
    println!("Example checksum: {}", example_checksum);
    assert_eq!(example_checksum, "01100");

    let part1_input = "01110110101001000";
    let part1_size = 272;
    {
        let part1_curve = generate_dragon(&as_vec(&part1_input), part1_size);
        let part1_checksum = checksum(&part1_curve);
        println!("Part 1 checksum: {}", part1_checksum);
        assert_eq!(part1_checksum, "11100111011101111");
    }

    // Part 2: larger disk
    {
        let part2_input = part1_input;
        let part2_size = 35651584;
        let part2_curve = generate_dragon(&as_vec(&part2_input), part2_size);
        let part2_checksum = checksum(&part2_curve);
        println!("Part 2 checksum: {}", part2_checksum);
        assert_eq!(part2_checksum, "10001110010000110");
    }
}
