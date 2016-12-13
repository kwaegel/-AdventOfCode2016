#![feature(step_by)]

use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::cmp;

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

//-----------------------------------------------------------------------------

// Materials stored as a bool array, in [chip, generator] pairs.
// Chip location = material_id*2,
// Gen  location = material_id*2 + 1
const FLOOR_SIZE: usize = 14;
#[derive(Copy,Clone,Debug)]
struct Floor {
    items: [bool; FLOOR_SIZE]
}
impl Floor {
    // A floor is "safe" if for each material_id*2, the next cell is also set.
    fn is_safe(&self) -> bool {

        let mut has_gen = false;
        for gen_id in (1..14).step_by(2) {
            if self.items[gen_id] {
                has_gen = true;
                break;
            }
        }
        // A floor without generators is always safe.
        if !has_gen {
            return true;
        }

        for chip_id in (0..14).step_by(2) {
            //println!("Checking [{},{}]", chip_id, chip_id+1);
            if self.items[chip_id] && !self.items[chip_id+1] {
//                println!("Not safe! [{}] = {}, [{}] = {}",
//                         chip_id, self.items[chip_id],
//                         chip_id+1, self.items[chip_id+1]);
                return false;
            }
        }
        true
    }
    fn add_chip(&mut self, item: Material) {
        println!("Adding chip in cell {}", item as usize*2);
        self.items[item as usize * 2] = true;
    }
    fn add_gen(&mut self, item: Material) {
        println!("Adding gen in cell {}", item as usize*2 + 1);
        self.items[item as usize * 2 + 1] = true;
    }
}

//-----------------------------------------------------------------------------

#[derive(Copy,Clone,Debug)]
struct Building {
    floors: [Floor; 4]
}
impl Building {
    fn new() -> Building {
        Building {
            floors: [
            Floor{items: [false; FLOOR_SIZE]},
            Floor{items: [false; FLOOR_SIZE]},
            Floor{items: [false; FLOOR_SIZE]},
            Floor{items: [false; FLOOR_SIZE]},
            ]
        }
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
        let sum0: usize = self.floors[0].items.iter().map(|&b| if b {1} else {0}).sum();
        let sum1: usize = self.floors[1].items.iter().map(|&b| if b {1} else {0}).sum();
        let sum2: usize = self.floors[2].items.iter().map(|&b| if b {1} else {0}).sum();
        (sum0+sum1+sum2) == 0
    }
    fn move_up(&mut self, floor: usize, item: usize) {
        if floor < 3 && self.floors[floor].items[item] {
            self.floors[floor].items[item] = false;
            self.floors[floor + 1].items[item] = true;
        }
    }
    fn move_down(&mut self, floor: usize, item: usize) {
        if floor > 0 && self.floors[floor].items[item] {
            self.floors[floor].items[item] = false;
            self.floors[floor - 1].items[item] = true;
        }
    }
}

//-----------------------------------------------------------------------------

const NO_PATH: usize = std::usize::MAX - 1;
const MAX_DEPTH: usize = 12;

// Returns the number of steps for everything to reach floor 4 (NO_PATH on failure)
fn process(input_state: &Building, current_floor: usize, depth: usize) -> usize {

    if depth > MAX_DEPTH {
        return NO_PATH;
    }

    if input_state.is_final() {
        return 0;
    }

    let mut fewest_steps = NO_PATH;

    // Pick pairs of items to move
    for item_1 in 0..FLOOR_SIZE {

        if !input_state.floors[current_floor].items[item_1] {
            continue;
        }

        // Starting the inner loop from item_1 cover the case of moving just one item.
        for item_2 in item_1..FLOOR_SIZE {
            if !input_state.floors[current_floor].items[item_2] {
                continue;
            }

            // Try moving both items up or down
            if current_floor < 3 {
                let steps = {
                    let mut next_state = input_state.clone();
                    next_state.move_up(current_floor, item_1);
                    next_state.move_up(current_floor, item_2);
                    if next_state.is_safe() {
                        process(&next_state, current_floor + 1, depth + 1) + 1
                    } else {
                        NO_PATH
                    }
                };
                fewest_steps = cmp::min(fewest_steps, steps);
            }

            if current_floor > 0 {
                let steps = {
                    let mut next_state = input_state.clone();
                    next_state.move_down(current_floor, item_1);
                    next_state.move_down(current_floor, item_2);
                    if next_state.is_safe() {
                        process(&next_state, current_floor - 1, depth + 1) + 1
                    } else {
                        NO_PATH
                    }
                };
                fewest_steps = cmp::min(fewest_steps, steps);
            }

        }
    }
//    if fewest_steps < NO_PATH {
//        println!("Path: {}", fewest_steps);
//    }

    fewest_steps
}

//-----------------------------------------------------------------------------

fn read_input(input: &str) -> Building {
    let regex_chip = Regex::new("([:alpha:]+)-compatible").unwrap();
    let regex_gen = Regex::new("([:alpha:]+) generator").unwrap();

    let mut building = Building::new();
    for (floor_idx, line) in input.lines().enumerate() {

        for cap in regex_chip.captures_iter(line) {
            let chip_type = cap.at(1).unwrap().parse::<Material>().unwrap();
            println!("Floor {}: {:?} type microchip", floor_idx, chip_type);
            building.floors[floor_idx].add_chip(chip_type);
        }

        for cap in regex_gen.captures_iter(line) {
            let gen_type = cap.at(1).unwrap().parse::<Material>().unwrap();
            println!("Floor {}: {:?} type generator", floor_idx, gen_type);
            building.floors[floor_idx].add_gen(gen_type);
        }
    }
    assert!(building.is_safe());
    building
}

//-----------------------------------------------------------------------------

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let  building = read_input(&input_string);
    let steps = process(&building, 0, 0);
    println!("Part 1: steps = {:?}", steps);
    assert!(steps == 11);
}

#[test]
fn test() {
    let test_input =
    "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.\n\
    The second floor contains a hydrogen generator.\n\
    The third floor contains a lithium generator.\n\
    The fourth floor contains nothing relevant.\n";

    let building = read_input(&test_input);
    let steps = process(&building, 0, 0);
    println!("Part 1: steps = {:?}", steps);
    assert!(steps == 11);
}