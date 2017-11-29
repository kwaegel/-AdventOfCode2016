
use std::fmt::Write;
use std::collections::VecDeque;

extern crate crypto;

mod utils;
use utils::*;

mod hasher;
use hasher::Hasher;

mod db;
use db::KeyDatabase;

fn compute_keys(salt: &str, iters: u32) -> Vec<u32> {

    let mut keys = Vec::new();

    // List of all quintuplets ever found.
    let mut quintuplet_list = KeyDatabase::new();

    // Values will be checked for quintuplets when added, and triplets when removed.
    let mut processing_queue: VecDeque<(u32, String)> = VecDeque::with_capacity(1002);

    let mut hasher = Hasher::new();

    // Loop through door_id + [0..max] to find hashes that contain quintuplets.
    for i in 0..u32::max_value() {
        let mut input_hash = String::new();
        let _ = write!(input_hash, "{}{}", salt, i);
        input_hash = hasher.hash(&input_hash, iters);

        // Add any entries to the quintuplet list.
        for chr in get_quintuplets(&input_hash) {
            //println!("Inserting {} at {}, hash {}", chr, i, input_hash);
            quintuplet_list.insert(chr, i);
        }

        // Push this entry on the processing queue.
        processing_queue.push_back((i, input_hash));

        // If we have at least 1000 elements in the queue, start checking for triples.
        while processing_queue.len() > 1001 {
            if let Some((index, hash)) = processing_queue.pop_front() {
                //println!("Popped [{}] {}", index, hash);

                // For each key K at i, check [i+1, i+1000].
                // E.g. for i=0, check range [1, 1000]
                if let Some(chr) = get_first_triplet(&hash) {
                    if let Some(_) = quintuplet_list.contains_in_range(chr, index + 1, index + 1000) {
                        keys.push(index);
                        break; // Don't double add to results.
                    }
                }
            }
        }

        if keys.len() >= 64 {
            break;
        }

    }

    keys
}


fn main() {

    // Test data
    {
        let keys = compute_keys("abc", 1);
        //println!("{} keys: {:?}", keys.len(), keys);
        assert!(keys.len() >= 64);
        println!("Part 1, example: index of 64-th key: {}", keys[63]);
        assert_eq!(keys[63], 22728);
    }


    // Part 1
    {
        let keys = compute_keys("ngcjuoqr", 1);
        //println!("{} keys: {:?}", keys.len(), keys);
        println!("Part 1: index of 64-th key: {}", keys[63]);
        assert!(keys.len() >= 64);
        assert_eq!(keys[63], 18626);
    }

    // Part 2
    {
        let keys = compute_keys("ngcjuoqr", 2017);
        //println!("{} keys: {:?}", keys.len(), &keys);
        println!("Part 2: index of 64-th key: {}", keys[63]);
        assert!(keys.len() >= 64);
        assert_eq!(keys[63], 20092);
    }
}

#[test]
fn part1_example() {
    let keys = compute_keys("abc", 1);
    assert!(keys.len() >= 64);
    println!("{} keys: {:?}", keys.len(), &keys[0..63]);
    assert_eq!(keys[0], 39);
    assert_eq!(keys[63], 22728);
}

#[test]
fn part2_example() {
    let keys = compute_keys("abc", 2017);
    println!("{} keys: {:?}", keys.len(), &keys[0..63]);
    assert!(keys.len() >= 64);
    assert_eq!(keys[63], 22551);
}
