
use std::fs::File;
use std::io::Read;
use std::cmp;
use std::fmt::Write;

// Clamp val to [lower, upper]
fn clamp<T: Ord>(val: T, lower: T, upper: T ) -> T {
    cmp::min(cmp::max(val, lower), upper)
}

#[derive(Eq,PartialEq,Copy,Clone,Hash,Debug)]
struct State {
    x: i32,
    y: i32
}

fn get_key(state: &State) -> i32 {
    let tupe = (state.x, state.y);
    match tupe {
        (-1, 1) => 1,
        (0, 1) => 2,
        (1, 1) => 3,
        (-1, 0) => 4,
        (0, 0) => 5,
        (1, 0) => 6,
        (-1, -1) => 7,
        (0, -1) => 8,
        (1, -1) => 9,
        _ => panic!("Invalid state"),
    }
}

// State is an integer pair in the range [-1,1][-1,1]
// indicating distance from the center of the keypad
// |1,2,3|
// |4,5,6|
// |7,8,9|
fn advance_state(prev: &State, input: &char) -> State {
    let mut next = match *input {
        'U' => State{x:prev.x, y: prev.y+1},
        'D' => State{x:prev.x, y: prev.y-1},
        'L' => State{x:prev.x-1, y: prev.y},
        'R' => State{x:prev.x+1, y: prev.y},
        _ => panic!("unknown input"),
    };
    next.x = clamp(next.x, -1, 1);
    next.y = clamp(next.y, -1, 1);
    next
}


fn process_sequence(input: &str ) -> String {
    let mut output = Vec::new();
    let mut state = State{x:0, y:0};
    for line in input.lines() {
        for c in line.chars() {
            state = advance_state(&state, &c);
        }
        let key = get_key(&state);
        output.push(key);
    }

    // Combine numbers into an output string
    let mut output_str = String::new();
    for c in output {
        let _ = write!(&mut output_str, "{}", c);
    }
    output_str
}

fn main() {

    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let code = process_sequence(&input_string);
    println!("Part 1 code: {}", code);
}

#[test]
fn test1() {
    let code = process_sequence("ULL\nRRDDD\nLURDL\nUUUUD");
    println!("{}", code);
    assert!(code == "1985");
}