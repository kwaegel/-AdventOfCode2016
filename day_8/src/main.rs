
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

mod matrix;
use matrix::Matrix2D;

fn process_instructions(input: &str, rows: usize, cols: usize) -> Matrix2D<bool> {
    let mut screen = Matrix2D::new_with_default(rows, cols, false);

    lazy_static! {
        static ref REGEX_RECT: Regex
            = Regex::new("rect ([:digit:]+)x([:digit:]+)").unwrap();


        static ref REGEX_ROTATE: Regex
            = Regex::new("rotate [:alpha:]+ ([xy])=([:digit:]+) by ([:digit:]+)").unwrap();
    }

    for line in input.lines() {
        if let Some(cap) = REGEX_RECT.captures(line) {
            let cols: usize = cap.at(1).unwrap().parse().unwrap();
            let rows: usize = cap.at(2).unwrap().parse().unwrap();
            // println!("Setting {} rows and {} cols", rows, cols);
            screen.set((0, 0), rows, cols, true);
            // println!("{}", screen);
        } else if let Some(cap) = REGEX_ROTATE.captures(line) {
            let row_or_col = cap.at(1).unwrap(); // x or y
            let index = cap.at(2).unwrap().parse::<usize>().unwrap();
            let amount = cap.at(3).unwrap().parse::<usize>().unwrap();
            // println!("moving {} at index {} by {}", row_or_col, index, amount);
            if row_or_col == "x" {
                // println!("rotating column {} down by {}", index, amount);
                screen.rotate_col_down(index, amount);
                // println!("{}", screen);
            } else {
                // row
                // println!("rotating row {} right by {}", index, amount);
                screen.rotate_row_right(index, amount);
                // println!("{}", screen);
            }
        } else {
            panic!("Unable to parse input");
        }
    }
    screen
}

fn count_lit(screen: Matrix2D<bool>) -> usize {
    let mut lit = 0usize;
    for &elm in screen.as_slice() {
        if elm {
            lit += 1;
        }
    }
    lit
}

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let screen = process_instructions(&input_string, 6, 50);
    println!("{}", screen);

    let lit = count_lit(screen);
    println!("Part 1: {} pixels lit", lit);
    assert!(lit = 121);

    // For part 2, just look at the console rendering of the screen.
}

#[test]
fn test() {
    let test_instructions = "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column \
                             x=1 by 1";

    let screen = process_instructions(test_instructions, 3, 7);
    println!("{}", screen);
    assert!(screen ==
            Matrix2D::from_rowmajor_vec(3,
                                        7,
                                        vec![false, true, false, false, true, false, true, true,
                                             false, true, false, false, false, false, false,
                                             true, false, false, false, false, false]));
}
