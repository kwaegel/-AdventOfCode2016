
/*
0,3      3,3
#########
#S| | | #
#-#-#-#-#
# | | | #
#-#-#-#-#
# | | | #
#-#-#-#-#
# | | |
####### V
0,0     3,0
*/

// Move from (0,3) -> (3,0)

extern crate crypto;

mod hasher;

#[derive(Debug)]
enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    fn from_val(val: usize) -> Dir {
        match val {
            0 => Dir::Up,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Right,
            _ => panic!("Unexpected direction code")
        }
    }
}

fn is_open(chr: char) -> bool {
    chr == 'b'
        || chr == 'c'
        || chr == 'd'
        || chr == 'e'
        || chr == 'f'
}

fn get_open_directions(code: &str) -> Vec<Dir> {
    let hash = hasher::hash(code);
    let directions = hash.chars()
        .take(4)
        .enumerate()
        .filter_map(|(idx, chr)|
            if is_open(chr) {
                Some(Dir::from_val(idx))
            } else {None})
        .collect::<Vec<_>>();
    //let first = hash.;
    directions
}

#[derive(Debug,Clone)]
struct State {
    pos: (i32, i32),
    state: String
}

impl State {
    fn new(code: &str) -> State {
        State{pos: (0,3), state: code.to_owned() }
    }

    fn is_valid(&self) -> bool {
        self.pos.0 >= 0 && self.pos.0 < 4 && self.pos.1 >= 0 && self.pos.1 < 4
    }

    fn is_target(&self) -> bool {
        self.pos == (3,0)
    }

    fn move_dir(&self, direction: &Dir) -> Option<State> {
        let next = match *direction {
            Dir::Up => State{pos: (self.pos.0, self.pos.1 + 1),
                state: self.state.to_owned() + "U"
            },
            Dir::Down => State{pos: (self.pos.0, self.pos.1 - 1),
                state: self.state.to_owned() + "D"
            },
            Dir::Left => State{pos: (self.pos.0 - 1, self.pos.1),
                state: self.state.to_owned() + "L"
            },
            Dir::Right => State{pos: (self.pos.0 + 1, self.pos.1),
                state: self.state.to_owned() + "R"
            },
        };

        if next.is_valid() { Some(next) } else {None}
    }
}

// Recursively search and find the target position
fn find_path(start: State) -> Option<State> {

    if start.is_target() {
        return Some(start);
    }

    //println!("Find path on {}", start.state);
    get_open_directions(&start.state)
        .iter()
        .filter_map(|dir| start.move_dir(dir))
        .filter_map(|next| find_path(next))
        .min_by_key(|state| state.state.len())
}

// Recursively search and find the target position
fn find_longest_path(start: State) -> Option<State> {

    if start.is_target() {
        return Some(start);
    }

    //println!("Find path on {}", start.state);
    get_open_directions(&start.state)
        .iter()
        .filter_map(|dir| start.move_dir(dir))
        .filter_map(|next| find_longest_path(next))
        .max_by_key(|state| state.state.len())
}

fn main() {
    {
        let example_start = State::new("hijkl");
        let min_path = find_path(example_start);
        //println!("{:?}", min_path);
        assert!(min_path.is_none());
    }

    {
        let example_start = State::new("ihgpwlah");
        let min_path = find_path(example_start);
        //println!("{:?}", min_path);
        assert_eq!(min_path.unwrap().state, "ihgpwlahDDRRRD");
    }

    {
        let example_start = State::new("kglvqrro");
        let min_path = find_path(example_start);
        //println!("{:?}", min_path);
        assert_eq!(min_path.unwrap().state, "kglvqrroDDUDRLRRUDRD");
    }

    {
        let example_start = State::new("ulqzkmiv");
        let min_path = find_path(example_start);
        //println!("{:?}", min_path);
        assert_eq!(min_path.unwrap().state, "ulqzkmivDRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    // Part 1
    let puzzle_input = "vwbaicqe";
    {
        let example_start = State::new(puzzle_input);
        let min_path = find_path(example_start);
        if let Some(min_path) = min_path {
            println!("Part 1: {}", &min_path.state[8..]);
            assert_eq!(min_path.state, "vwbaicqeDRDRULRDRD");
        }
    }

    // Part 2

    {
        let example_start = State::new("ihgpwlah");
        let max_path = find_longest_path(example_start);
        assert!(max_path.is_some());
        if let Some(max_path) = max_path {
            let length = max_path.state.len() - puzzle_input.len();
            //println!("max path length: {}", length);
            assert_eq!(length, 370);
        }
    }

    {
        let example_start = State::new("ulqzkmiv");
        let max_path = find_longest_path(example_start);
        assert!(max_path.is_some());
        if let Some(max_path) = max_path {
            let length = max_path.state.len() - puzzle_input.len();
            //println!("max path length: {}", length);
            assert_eq!(length, 830);
        }
    }

    {
        let example_start = State::new(puzzle_input);
        let max_path = find_longest_path(example_start);
        if let Some(max_path) = max_path {
            let length = max_path.state.len() - puzzle_input.len();
            println!("Part 2: max path length: {}", length);
            assert_eq!(length, 384);
        }
    }
}
