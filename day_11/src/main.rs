
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::cmp;
use std::fmt;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

mod bit_floor;
use bit_floor::BitFloor;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Material {
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
const FLOOR_SIZE: usize = 11;

// -----------------------------------------------------------------------------

#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
struct Building {
    floors: [BitFloor; 4],
    elevator_idx: usize,
    depth: usize,
}
impl Building {
    fn new() -> Building {
        Building {
            floors: [BitFloor::new(); 4],
            elevator_idx: 0,
            depth: 0
        }
    }

    fn item_exists(&self, item: usize) -> bool {
        self.floors[self.elevator_idx].is_set(item)

    }

    fn is_safe(&self) -> bool {
        self.floors.iter().all(|&floor| floor.is_safe())
    }

    // Number of items-steps to reach the top floor.
    fn distance(&self) -> u32 {
        self.floors[2].num_items() * 1 +
        self.floors[1].num_items() * 2 +
        self.floors[0].num_items() * 3
    }

    // Check if everything is on the fourth floor (floors 0-2 are empty)
    fn is_final(&self) -> bool {
        self.floors[0].is_empty()
        && self.floors[1].is_empty()
        && self.floors[2].is_empty()
    }

    // Try to move one or more items up a floor and return the new building state.
    fn try_move_up(&self, item_1: usize, item_2: usize) -> Option<Building> {
        if self.elevator_idx < 3 && self.floors[self.elevator_idx].is_set(item_1) &&
           self.floors[self.elevator_idx].is_set(item_2) {
            let mut next = self.clone();

            next.floors[next.elevator_idx].clear(item_1);
            next.floors[next.elevator_idx + 1].set(item_1);

            next.floors[next.elevator_idx].clear(item_2);
            next.floors[next.elevator_idx + 1].set(item_2);

            next.elevator_idx += 1;
            next.depth = self.depth+1;
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
        if self.elevator_idx > 0 && self.floors[self.elevator_idx].is_set(item_1) &&
           self.floors[self.elevator_idx].is_set(item_2) {
            let mut next = self.clone();

            next.floors[next.elevator_idx].clear(item_1);
            next.floors[next.elevator_idx - 1].set(item_1);

            next.floors[next.elevator_idx].clear(item_2);
            next.floors[next.elevator_idx - 1].set(item_2);

            next.elevator_idx -= 1;
            next.depth = self.depth+1;
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
                write!(f, "{}", if self.floors[n].is_set(item) {"# "} else {". "})?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "Distance {}", self.distance())
        //write!(f, "")
    }
}

// For sorting in a priority queue.
impl Ord for Building {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Normally "(self.distance()).cmp(&other.distance())", but inverted for a min-heap.
        (other.distance()).cmp(&self.distance())
    }
}

impl PartialOrd for Building {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// -----------------------------------------------------------------------------

const NO_PATH: usize = std::usize::MAX - 1;
const MAX_DEPTH: usize = 20;

// Returns the number of steps for everything to reach floor 4 (NO_PATH on failure)
// fn process(input_state: &Building, current_floor: usize, depth: usize) -> usize {

// Input: stack of building states, the current one at states[states.len()-1];
// Returns: size of the stack when is_final() on the last state is true.
fn process_dfs(states: &mut Vec<Building>, max_depth: usize) -> usize {

    let current_idx = states.len() - 1;

//    println!("Checking state {}", current_idx);
//    println!("Checking state\n{}", states[current_idx]);

    if states.len() > max_depth {
        return NO_PATH;
    }

    if states[current_idx].is_final() {
        // Print the list of moves for debugging

//        println!("Found solution at state vector size {}", states.len());
//        for state in states {
//            println!("{}", state);
//        }
//        panic!("ended early for debugging");

        //println!("Found solution at state vector size {}", states.len());
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
                let is_cycle = states.contains(&next_state);

                if !is_cycle {
                    states.push(next_state);
                    let steps = process_dfs(states, max_depth);
                    fewest_steps = cmp::min(fewest_steps, steps);
                    states.pop();
                }
            }

            let next_down = states[current_idx].try_move_down(item_1, item_2);
            if let Some(next_state) = next_down {

                // Check against prior state back to avoid backtracking.
                let is_cycle = states.contains(&next_state);
                if !is_cycle {
                    states.push(next_state);
                    let steps = process_dfs(states, max_depth);
                    fewest_steps = cmp::min(fewest_steps, steps);
                    states.pop();
                }
            }
        }
    }
    fewest_steps
}

// -----------------------------------------------------------------------------

// Returns the number of steps for everything to reach floor 4 (NO_PATH on failure)
fn process_recursive(input_state: &Building, depth: usize, history: &mut HashSet<Building>, max_depth: usize) -> usize {

    if depth > max_depth {
        return NO_PATH;
    }

    if input_state.is_final() {
        return 0;
    }

    let mut fewest_steps = NO_PATH;

    // Pick pairs of items to move
    for item_1 in 0..FLOOR_SIZE {

        if !input_state.item_exists(item_1) {
            continue;
        }

        // Starting the inner loop from item_1 cover the case of moving just one item.
        for item_2 in item_1..FLOOR_SIZE {
            if !input_state.item_exists(item_2) {
                continue;
            }


            // Try moving both items up or down
            if let Some(next_state) = input_state.try_move_up(item_1, item_2) {
                if !history.contains(&next_state) {
                    let steps = process_recursive(&next_state, depth + 1, history, max_depth) + 1;
                    fewest_steps = cmp::min(fewest_steps, steps);
                }
            }

            // Try moving both items up or down
            if let Some(next_state) = input_state.try_move_down(item_1, item_2) {
                if !history.contains(&next_state) {
                    let steps = process_recursive(&next_state, depth + 1, history, max_depth) + 1;
                    fewest_steps = cmp::min(fewest_steps, steps);
                }
            }
        }
    }
        if fewest_steps < NO_PATH {
            println!("Path: {}", fewest_steps);
        }

    fewest_steps
}

// -----------------------------------------------------------------------------

// Iterative deepening
fn process_id(states: &mut Vec<Building>) -> usize {
    for i in 1..20 {
        println!("Trying depth {}...", i);
        let steps = process_dfs(states, i);
        if steps < NO_PATH {
            return steps;
        }
    }
    NO_PATH
}

// -----------------------------------------------------------------------------

fn process_bfs(initial_state: &Building) -> usize {

    let mut queue = VecDeque::new();
    queue.push_back(*initial_state);

    // Prevent cycles
    let mut history = HashSet::new();
    history.insert(*initial_state);

    // Effectively an infinite loop, given the size of the search space.
    while let Some(current_state) = queue.pop_front() {
        if current_state.is_final() {
            return current_state.depth;
        }

        // Generate future states by moving one or two items
        // When item_2 == item_1, we only move one item.
        for item_1 in 0..FLOOR_SIZE {
            if !current_state.item_exists(item_1) { continue; }
            for item_2 in item_1..FLOOR_SIZE {
                if !current_state.item_exists(item_2) { continue; }

                // println!("Moving items {} and {}", item_1, item_2);

                if let Some(next) = current_state.try_move_up(item_1, item_2) {
                    if !history.contains(&next) {
                        queue.push_back(next);
                    }
                }

                if let Some(next) = current_state.try_move_down(item_1, item_2) {
                    if !history.contains(&next) {
                        queue.push_back(next);
                    }
                }
            }
        }

    } // end queue loop
    NO_PATH
}

// -----------------------------------------------------------------------------

fn process_best_first(initial_state: &Building) -> usize {

    let mut queue = BinaryHeap::new();
    queue.push(*initial_state);

    // Prevent cycles
    let mut history = HashSet::new();
    history.insert(*initial_state);

    // Effectively an infinite loop, given the size of the search space.
    while let Some(current_state) = queue.pop() {

        println!("testing\n{}", &current_state);

        if current_state.is_final() {
            return current_state.depth;
        }

        // Generate future states by moving one or two items
        // When item_2 == item_1, we only move one item.
        for item_1 in 0..FLOOR_SIZE {
            if !current_state.item_exists(item_1) { continue; }
            for item_2 in item_1..FLOOR_SIZE {
                if !current_state.item_exists(item_2) { continue; }

                // println!("Moving items {} and {}", item_1, item_2);

                if let Some(next) = current_state.try_move_up(item_1, item_2) {
                    if !history.contains(&next) {
                        queue.push(next);
                    }
                }

                if let Some(next) = current_state.try_move_down(item_1, item_2) {
                    if !history.contains(&next) {
                        queue.push(next);
                    }
                }
            }
        }

    } // end queue loop
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

    println!("Searching for solution...");
    let steps = process_bfs(&building);

//    let mut states = Vec::new();
//    unsafe { states.reserve(MAX_DEPTH+1); }
//    states.push(building);
//
//    println!("Searching for solution...");
//    let mut steps = process_id(&mut states);
//    steps -= 1; // Subtract 1 for the initial state.
//    //let steps = process_recursive(&building, 0);

    println!("Part 1: steps = {:?}", steps);
    //assert!(steps == 11);
}

// -----------------------------------------------------------------------------

#[test]
fn test_example_input() {
    let test_input = "The first floor contains a hydrogen-compatible microchip and a \
                      lithium-compatible microchip.\nThe second floor contains a hydrogen \
                      generator.\nThe third floor contains a lithium generator.\nThe fourth floor \
                      contains nothing relevant.\n";

    let building = read_input(&test_input);
    assert!(building.is_safe());

    println!("Building:\n{}", &building);

    let mut states = Vec::new();
    states.reserve(MAX_DEPTH+1);
    states.push(building);

    println!("Searching for solution...");
    //let mut steps = process_id(&mut states);
    //steps -= 1; // Subtract 1 for the initial state.

    let mut history = HashSet::new();
    let steps = process_recursive(&building, 0, &mut history, MAX_DEPTH);

    println!("Example data: steps = {:?}", steps);
    assert!(steps < NO_PATH);
    assert!(steps == 11);
}

#[test]
fn test_recursive() {
    let test_input = "The first floor contains a hydrogen-compatible microchip and a \
                      lithium-compatible microchip.\nThe second floor contains a hydrogen \
                      generator.\nThe third floor contains a lithium generator.\nThe fourth floor \
                      contains nothing relevant.\n";

    let building = read_input(&test_input);
    assert!(building.is_safe());

    println!("Building:\n{}", &building);

    let mut states = Vec::new();
    unsafe { states.reserve(MAX_DEPTH+1); }
    states.push(building);

    println!("Searching for solution...");
    let mut history = HashSet::new();
    let steps = process_recursive(&building, 0, &mut history, MAX_DEPTH);

    println!("Example data: steps = {:?}", steps);
    assert!(steps < NO_PATH);
    assert!(steps == 11);
}

#[test]
fn test_bfs() {
    let test_input = "The first floor contains a hydrogen-compatible microchip and a \
                      lithium-compatible microchip.\nThe second floor contains a hydrogen \
                      generator.\nThe third floor contains a lithium generator.\nThe fourth floor \
                      contains nothing relevant.\n";

    let building = read_input(&test_input);
    assert!(building.is_safe());

    println!("Building:\n{}", &building);
    println!("Searching for solution...");

    let steps = process_bfs(&building);

    println!("Example data: steps = {:?}", steps);
    assert!(steps < NO_PATH);
    assert!(steps == 11);
}

#[test]
fn test_best_first() {
    let test_input = "The first floor contains a hydrogen-compatible microchip and a \
                      lithium-compatible microchip.\nThe second floor contains a hydrogen \
                      generator.\nThe third floor contains a lithium generator.\nThe fourth floor \
                      contains nothing relevant.\n";

    let building = read_input(&test_input);
    assert!(building.is_safe());

    println!("Building:\n{}", &building);
    println!("Searching for solution...");

    let steps = process_best_first(&building);

    println!("Example data: steps = {:?}", steps);
    assert!(steps < NO_PATH);
    assert!(steps == 11);
}


#[test]
fn test_safe() {
    let mut building = Building::new();

    building.floors[1].add_gen(Material::Hydrogen);
    building.floors[1].add_chip(Material::Hydrogen);
    building.floors[1].add_chip(Material::Lithium);

    building.floors[2].add_gen(Material::Lithium);

    //println!("Building:\n{}", &building);

    assert!(!building.floors[1].is_safe());

    assert!(!building.is_safe());
}

#[test]
fn test_not_safe() {
    let mut building = Building::new();

    building.floors[1].add_gen(Material::Hydrogen);
    building.floors[1].add_chip(Material::Hydrogen);
    building.floors[0].add_chip(Material::Lithium);

    building.floors[2].add_gen(Material::Lithium);

    //println!("Building:\n{}", &building);

    assert!(building.floors[1].is_safe());

    assert!(building.is_safe());
}
