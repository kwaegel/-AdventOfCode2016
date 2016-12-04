
use std::fs::File;
use std::io::Read;
use std::fmt::Write;

mod matrix; // Define that the file matrix.rs contains mod matrix
use matrix::Matrix;

#[derive(PartialEq,Copy,Clone,Debug)]
struct State {
    x: i32,
    y: i32
}

// State is an integer pair indicating distance from upper left corner of the grid.
fn advance_state_on_grid(prev: &State, input: &char, grid: &Matrix<char>) -> State {
    let next = match *input {
        'U' => State{x:prev.x, y: prev.y-1},
        'D' => State{x:prev.x, y: prev.y+1},
        'L' => State{x:prev.x-1, y: prev.y},
        'R' => State{x:prev.x+1, y: prev.y},
        _ => panic!("unknown input"),
    };
    if grid.is_valid(next.y, next.x, '-') {next} else {*prev}
}

fn process_sequence_on_grid(initial_state: &State,
                            input: &str,
                            grid: &Matrix<char>) -> String {
    let mut output = Vec::new();
    let mut state = *initial_state;
    for line in input.lines() {
        for c in line.chars() {
            state = advance_state_on_grid(&state, &c, &grid);
        }
        let key = grid.at(state.y as usize, state.x as usize);
        output.push(key);
    }

    // Combine keys into an output string
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

    let simple_keypad = Matrix::from_array(3,3,
        &['1','2','3',
          '4','5','6',
          '7','8','9']);

    let initial_state1 = State{x:0, y:0};
    let code1 = process_sequence_on_grid(&initial_state1, &input_string, &simple_keypad);
    println!("Part 1 code: {}", code1);
    assert!(code1 == "78985");


    let complex_keypad = Matrix::from_array(5,5,
        &['-','-','1','-','-',
          '-','2','3','4','-',
          '5','6', '7','8','9',
          '-','A','B','C','-',
          '-','-','D','-','-']);

    let initial_state2 = State{x:0, y:2}; // Key 5
    let code2 = process_sequence_on_grid(&initial_state2, &input_string, &complex_keypad);
    println!("Part 2 code: {}", code2);
    assert!(code2 == "57DD8");
}

#[test]
fn test_simple() {

    let keypad = Matrix::from_array(3,3,
                 &['1','2','3',
                  '4','5','6',
                  '7','8','9']);

    let initial_state = State{x:1, y:1};
    let code = process_sequence_on_grid(&initial_state,
                                        "ULL\nRRDDD\nLURDL\nUUUUD",
                                        &keypad);
    println!("{}", code);
    assert!(code == "1985");
}

#[test]
fn test_complex() {

    let keypad = Matrix::from_array(5,5,
        &['-','-','1','-','-',
          '-','2','3','4','-',
          '5','6', '7','8','9',
          '-','A','B','C','-',
          '-','-','D','-','-']);

    let initial_state = State{x:0, y:2}; // Key 5
    let code = process_sequence_on_grid(&initial_state,
                                        "ULL\nRRDDD\nLURDL\nUUUUD",
                                        &keypad);
    println!("{}", code);
    assert!(code == "5DB3");
}