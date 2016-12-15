#![feature(step_by)]

use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::cmp;
use std::fmt;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Material {
    Hydrogen = 0,
    Lithium = 1,
    Polonium = 2,
    Thulium = 3,
    Promethium = 4,
    Ruthenium = 5,
    Cobalt = 6,
}
impl FromStr for Material {
    type Err = ();

    fn from_str(s: &str) -> Result<Material, ()> {
        match s {
            "hydrogen" => Ok(Material::Hydrogen),
            "lithium" => Ok(Material::Lithium),
            "polonium" => Ok(Material::Polonium),
            "thulium" => Ok(Material::Thulium),
            "promethium" => Ok(Material::Promethium),
            "ruthenium" => Ok(Material::Ruthenium),
            "cobalt" => Ok(Material::Cobalt),
            _ => Err(()),
        }
    }
}

// -----------------------------------------------------------------------------

// Materials stored as a bool array, in [generator, chip] pairs.
// Gen  location = material_id*2
// Chip location = material_id*2 + 1,
const FLOOR_SIZE: usize = 14;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
struct Floor {
    items: [bool; FLOOR_SIZE],
}
impl Floor {
    fn new() -> Floor {
        Floor { items: [false; FLOOR_SIZE] }
    }
    // A floor is "safe" if for each material_id*2, the previous cell is also set.
    fn is_safe(&self) -> bool {

        let mut has_gen = false;
        for gen_id in (0..14).step_by(2) {
            if self.items[gen_id] {
                has_gen = true;
                break;
            }
        }
        // A floor without generators is always safe.
        if !has_gen {
            //println!("Safe: no generators");
            return true;
        }

        for gen_id in (0..14).step_by(2) {
            let has_gen = self.items[gen_id];
            let has_chip = self.items[gen_id+1];
            if has_chip && !has_gen  {
//                println!("Not safe! gen [{}] = {}, chip [{}] = {}",
//                         gen_id, self.items[gen_id],
//                         gen_id+1, self.items[gen_id+1]);
                return false;
            }
        }
        true
    }
    fn add_chip(&mut self, item: Material) {
        //println!("Adding chip in cell {}", item as usize * 2+1);
        self.items[item as usize * 2 + 1] = true;
    }
    fn add_gen(&mut self, item: Material) {
        //println!("Adding gen in cell {}", item as usize * 2);
        self.items[item as usize * 2] = true;
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
struct Building {
    floors: [Floor; 4],
    elevator_idx: usize,
}
impl Building {
    fn new() -> Building {
        Building {
            floors: [Floor::new(); 4],
            elevator_idx: 0,
        }
    }

    fn item_exists(&self, item: usize) -> bool {
        self.floors[self.elevator_idx].items[item]
    }

    fn is_safe(&self) -> bool {
        for floor in self.floors.iter() {
            if !floor.is_safe() {
                return false;
            }
        }
        true
    }
    // Check if everything is on the fourth floor (floors 0-2 are empty)
    fn is_final(&self) -> bool {
        let sum: usize = self.floors[3].items.iter().map(|&b| if b { 1 } else { 0 }).sum();
        sum == 4 || sum == 10 // 4 for example data, 10 for input file
    }

    // Try to move one or more items up a floor and return the new building state.
    fn try_move_up(&self, item_1: usize, item_2: usize) -> Option<Building> {
        if self.elevator_idx < 3 && self.floors[self.elevator_idx].items[item_1] &&
           self.floors[self.elevator_idx].items[item_2] {
            let mut next = self.clone();

            next.floors[next.elevator_idx].items[item_1] = false;
            next.floors[next.elevator_idx + 1].items[item_2] = true;

            next.floors[next.elevator_idx].items[item_2] = false;
            next.floors[next.elevator_idx + 1].items[item_1] = true;

            next.elevator_idx += 1;
            if next.is_safe() {
                Some(next)
            } else {
                None
            }
        } else {
            None
        }
    }

    // Try to move one or more items up a floor and return the new building state.
    fn try_move_down(&self, item_1: usize, item_2: usize) -> Option<Building> {
        if self.elevator_idx > 0 && self.floors[self.elevator_idx].items[item_1] &&
           self.floors[self.elevator_idx].items[item_2] {
            let mut next = self.clone();

            next.floors[next.elevator_idx].items[item_1] = false;
            next.floors[next.elevator_idx - 1].items[item_2] = true;

            next.floors[next.elevator_idx].items[item_2] = false;
            next.floors[next.elevator_idx - 1].items[item_1] = true;

            next.elevator_idx -= 1;
            if next.is_safe() {
                Some(next)
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            let n = 3-i;
            write!(f, "F{} {} ", n, if n==self.elevator_idx {"E"} else {"."})?;
            for item in 0..FLOOR_SIZE {
                write!(f, "{}", if self.floors[n].items[item] {"# "} else {". "})?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

// -----------------------------------------------------------------------------

const NO_PATH: usize = std::usize::MAX - 1;
static mut MAX_DEPTH: usize = 50;

// Returns the number of steps for everything to reach floor 4 (NO_PATH on failure)
// fn process(input_state: &Building, current_floor: usize, depth: usize) -> usize {

// Input: stack of building states, the current one at states[states.len()-1];
// Returns: size of the stack when is_final() on the last state is true.
fn process(states: &mut Vec<Building>) -> usize {

    let current_idx = states.len() - 1;

    // println!("Checking state {}", current_idx);

    unsafe {
        if states.len() > MAX_DEPTH {
            return NO_PATH;
        }
    }

    if states[current_idx].is_final() {
        // Print the list of moves for debugging

//        println!("Found solution at state vector size {}", states.len());
//        for state in states {
//            println!("{}", state);
//        }
//        panic!("ended early for debugging");


        println!("Found solution at state vector size {}", states.len());

        return states.len();
    }

    let mut fewest_steps = NO_PATH;

    // Pick pairs of items to move
    for item_1 in 0..FLOOR_SIZE {
        // Need at least one item to move
        if !states[current_idx].item_exists(item_1) {
            continue;
        }

        // When item_2 == item_1, we only move one item.
        for item_2 in item_1..FLOOR_SIZE {
            if !states[current_idx].item_exists(item_2) {
                continue;
            }

            // println!("Moving items {} and {}", item_1, item_2);

            // Try moving both items up or down
            let next_up = states[current_idx].try_move_up(item_1, item_2);
            if let Some(next_state) = next_up {
                // Check against prior state back to avoid backtracking.
                if current_idx <= 1 || next_state != states[current_idx - 1] {
                    states.push(next_state);
                    let steps = process(states);
                    fewest_steps = cmp::min(fewest_steps, steps);
                    states.pop();
                }
            }

            let next_down = states[current_idx].try_move_down(item_1, item_2);
            if let Some(next_state) = next_down {
                // Check against prior state back to avoid backtracking.
                if current_idx <= 1 || next_state != states[current_idx - 1] {
                    states.push(next_state);
                    let steps = process(states);
                    fewest_steps = cmp::min(fewest_steps, steps);
                    states.pop();
                }
            }
        }
    }
    fewest_steps
}

// -----------------------------------------------------------------------------

// Iterative deepening
fn process_id(states: &mut Vec<Building>) -> usize {
    for i in 3..20 {
        println!("Trying depth {}...", i);
        unsafe { MAX_DEPTH = i; }
        let steps = process(states);
        if steps < NO_PATH {
            return steps;
        }
    }
    NO_PATH
}

// -----------------------------------------------------------------------------

fn read_input(input: &str) -> Building {
    let regex_chip = Regex::new("([:alpha:]+)-compatible").unwrap();
    let regex_gen = Regex::new("([:alpha:]+) generator").unwrap();

    let mut building = Building::new();
    for (floor_idx, line) in input.lines().enumerate() {

        for cap in regex_chip.captures_iter(line) {
            let chip_type = cap.at(1).unwrap().parse::<Material>().unwrap();
            //println!("Floor {}: {:?} type microchip", floor_idx, chip_type);
            building.floors[floor_idx].add_chip(chip_type);
        }

        for cap in regex_gen.captures_iter(line) {
            let gen_type = cap.at(1).unwrap().parse::<Material>().unwrap();
            //println!("Floor {}: {:?} type generator", floor_idx, gen_type);
            building.floors[floor_idx].add_gen(gen_type);
        }
    }
    assert!(building.is_safe());
    building
}

// -----------------------------------------------------------------------------

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let building = read_input(&input_string);

    let mut states = Vec::new();
    unsafe { states.reserve(MAX_DEPTH+1); }
    states.push(building);

    println!("Searching for solution...");
    let mut steps = process_id(&mut states);
    steps -= 1; // Subtract 1 for the initial state.

    println!("Part 1: steps = {:?}", steps);
    assert!(steps == 11);
}

#[test]
fn test_example_input() {
    let test_input = "The first floor contains a hydrogen-compatible microchip and a \
                      lithium-compatible microchip.\nThe second floor contains a hydrogen \
                      generator.\nThe third floor contains a lithium generator.\nThe fourth floor \
                      contains nothing relevant.\n";

    let building = read_input(&test_input);
    assert!(building.is_safe());

    //println!("Building:\n{}", &building);

    let mut states = Vec::new();
    unsafe { states.reserve(MAX_DEPTH+1); }
    states.push(building);

    println!("Searching for solution...");
    let mut steps = process_id(&mut states);
    steps -= 1; // Subtract 1 for the initial state.

    println!("Example data: steps = {:?}", steps);
    assert!(steps < NO_PATH);
    assert!(steps == 11);
}

#[test]
fn safety_test() {
    let mut building = Building::new();

    building.floors[1].add_chip(Material::Hydrogen);
    building.floors[1].add_gen(Material::Hydrogen);
    building.floors[1].add_chip(Material::Lithium);

    building.floors[2].add_gen(Material::Lithium);

    println!("Building:\n{}", &building);
    assert!(!building.is_safe());
}
