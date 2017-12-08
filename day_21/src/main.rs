
#![feature(slice_rotate)]

use std::num::ParseIntError;

mod input;
use input::*;

// Possible string functions:
// swap position X with position Y
//    means that the letters at indexes X and Y (counting from 0) should be swapped.
fn swap_pos(input: &mut [char], x: usize, y: usize) {
    input.swap(x, y);
}

// swap letter X with letter Y
//    means that the letters X and Y should be swapped (regardless of where they appear in
//    the string).
fn swap_letter(input: &mut [char], x: char, y: char) {
    let x_idx = input.iter().position(|&c| c==x).unwrap();
    let y_idx = input.iter().position(|&c| c==y).unwrap();
    swap_pos(input, x_idx, y_idx);
}
// rotate left/right X steps
//     means that the whole string should be rotated; for example, one right rotation would
//     turn abcd into dabc.
fn rotate_right(input: &mut [char], amount: usize) {
    let mid = input.len() - (amount % input.len());
    input.rotate(mid);
}
fn rotate_left(input: &mut [char], amount: usize) {
    input.rotate(amount);
}
// rotate based on position of letter X
//     means that the whole string should be rotated to the right based on the index of letter X
//     (counting from 0) as determined before this instruction does any rotations. Once the index
//     is determined, rotate the string to the right one time, plus a number of times equal to
//     that index, plus one additional time if the index was at least 4.
fn rotate_by_index(input: &mut [char], x: char) {
    let index = input.iter().position(|&c| c==x).unwrap();
    let mut amount = index + 1;
    if index >= 4 {
        amount += 1;
    }
    println!("rotating by {}", amount);
    rotate_right(input, amount);
    //input.rotate(amount);
}
// reverse positions X through Y
//     means that the span of letters at indexes X through Y (including the letters at X and Y)
//     should be reversed in order.
fn reverse_range(input: &mut [char], x: usize, y: usize) {
    let range = &mut input[x..y+1];
    range.reverse();
}
// move position X to position Y
//     means that the letter which is at index X should be removed from the string, then inserted
//     such that it ends up at index Y.
fn move_to_pos(input: &mut [char], x: usize, y: usize) {

    let mut i = x;
    while i < y {
        swap_pos(input, i, i+1);
        i += 1;
    }
    while i > y {
        swap_pos(input, i, i-1);
        i -= 1;
    }
}


// swap position X with position Y
// swap letter X with letter Y
// rotate left/right X steps
// rotate based on position of letter X
// reverse positions X through Y
// move position X to position Y
fn parse_instructions(instructions: &str, mut input: &mut [char]) -> Result<(), ParseIntError> {
    for line in instructions.lines() {
        let splits: Vec<&str> = line.split_whitespace().collect();

        //println!("{:?}", splits);

        if splits[0] == "swap" && splits[1] == "position" {
            let x: usize = splits[2].parse()?;
            let y: usize = splits[5].parse()?;
            swap_pos(&mut input, x,y);

        } else if splits[0] == "swap" && splits[1] == "letter" {
            let x = splits[2].chars().next().unwrap();
            let y = splits[5].chars().next().unwrap();
            swap_letter(&mut input, x,y);

        } else if splits[0] == "rotate" && splits[1] == "based" {
            let letter = splits[6].chars().next().unwrap();
            rotate_by_index(&mut input, letter);

        } else if splits[0] == "rotate" && splits[1] == "right" {
            let amount: usize = splits[2].parse()?;
            rotate_right(&mut input, amount);

        } else if splits[0] == "rotate" && splits[1] == "left" {
            let amount: usize = splits[2].parse()?;
            rotate_left(&mut input, amount);

        } else if splits[0] == "reverse" {
            let x: usize = splits[2].parse()?;
            let y: usize = splits[4].parse()?;
            reverse_range(&mut input, x,y);

        } else if splits[0] == "move" {
            let x: usize = splits[2].parse()?;
            let y: usize = splits[5].parse()?;
            move_to_pos(&mut input, x,y);
        } else {
            panic!("Unknown instruction!");
        }

        print(&input);
    }

    Ok(())
}


fn print(input: &[char]) {
    let s = input.iter().cloned().collect::<String>();
    println!("{}", s);
}

fn main() {

    //let mut example: Vec<_> = "abcde".chars().collect();
    //parse_instructions(&EXAMPLE_INPUT, &mut example).unwrap();

    let mut puzle_part_1: Vec<_> = "abcdefgh".chars().collect();
    parse_instructions(&PUZZLE_INPUT, &mut puzle_part_1).unwrap();
    print(&puzle_part_1);
    assert_eq(&part_1, "hcdefbag");
}
