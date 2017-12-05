
mod input;

#[allow(dead_code)]
fn print_vec(v: &Vec<(u32,u32)>) {
    v.iter()
        .inspect(|&val| println!("{}-{}", val.0, val.1))
        .count();
}

// Parse a list of \n separated closed ranges of u32 numbers from a string.
fn parse_ranges(range_list: &str) -> Vec<(u32, u32)> {

    let mut ranges = Vec::new();

    // Each range is separated by a newline symbol.
    for range in range_list.lines() {
        let values: Vec<u32> = range
            .split('-')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        ranges.push((values[0], values[1]));
    }
    ranges
}

// Merge all overlapping ranges.
fn merge_ranges(input_ranges: &Vec<(u32, u32)> ) -> Vec<(u32, u32)> {

    let mut ranges = input_ranges.clone();
    ranges.sort_by_key(|k| k.0);
    //println!("Sorted: {:?}", ranges);

    let mut i=0;
    while i < ranges.len()-1 {
        // Merge if ranges overlap
        if ranges[i].1 >= ranges[i+1].0 - 1 {
            let low = ranges[i].0;
            let high = ranges[i].1.max(ranges[i+1].1);
            ranges[i] = (low, high);
            ranges.remove(i+1);
        } else {
            // Check the next range
            i += 1;
        }
    }

    ranges
}

// Count the values outside the provided ranges.
fn count_open(ranges: &Vec<(u32, u32)>, max: u32) -> u32 {

    let mut count = ranges[0].0;
    for i in 0..ranges.len()-1 {
        // Calculate size of the half-open range [lhs, rhs)
        let lhs = ranges[i].1+1;
        let rhs = ranges[i+1].0;
        count += rhs - lhs;
    }

    if ranges.last().unwrap().1 < max {
        let last_blocked = ranges.last().unwrap().1;
        count += max - last_blocked;
    }

    count
}



fn main() {
//    {
//        let ranges = parse_ranges(input::EXAMPLE_INPUT);
//        let merged_ranges = merge_ranges(&ranges);
//        println!("Example: {:?}", merged_ranges);
//        let low_address = merged_ranges[0].1+1;
//        println!("Example: first open address: {}", low_address);
//        //let open = count_open(&merged_ranges, 9);
//        //println!("Open addresses: {}", open);
//
//        let blocked = count_blocked(&merged_ranges);
//        println!("Blocked addresses: {}", blocked);
//        let free = 9 - blocked;
//        println!("Free addresses: {}", free);
//    }

//    println!();
//
//    {
//        let ranges = parse_ranges(input::EXAMPLE_INPUT_2);
//        let merged_ranges = merge_ranges(&ranges);
//        println!("Example 2: {:?}", merged_ranges);
//
//        let low_address = merged_ranges[0].1+1;
//        println!("Example 2: first open address: {}", low_address);
//
//        let open = count_open(&merged_ranges, 9);
//        println!("Example 2: free addresses: {}", open);
//
//        let blocked = count_blocked(&merged_ranges);
//        println!("Blocked addresses: {}", blocked);
//        let max = 9;
//        let free = max - blocked + 1;
//        println!("Free addresses: {}", free);
//    }
//
//    println!();
//
    {
        let ranges = parse_ranges(input::PUZZLE_INPUT);
        let merged_ranges = merge_ranges(&ranges);
        println!("Part 1: {} merged ranges", merged_ranges.len());
        //print_vec(&merged_ranges);

        let low_address = merged_ranges[0].1+1;
        println!("Part 1: first open address: {}", low_address);
        assert_eq!(low_address, 32259706);
        let open = count_open(&merged_ranges, 4294967295);
        println!("Free addresses: {}", open);
        assert_eq!(open, 113);
    }
}
