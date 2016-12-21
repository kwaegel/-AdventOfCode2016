
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::VecDeque;

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
    Elerium = 7,
    Dilithium = 8,
}
impl FromStr for Material {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Material, &'static str> {
        match s {
            "hydrogen" => Ok(Material::Hydrogen),
            "lithium" => Ok(Material::Lithium),
            "polonium" => Ok(Material::Polonium),
            "thulium" => Ok(Material::Thulium),
            "promethium" => Ok(Material::Promethium),
            "ruthenium" => Ok(Material::Ruthenium),
            "cobalt" => Ok(Material::Cobalt),
            "elerium" => Ok(Material::Elerium),
            "dilithium" => Ok(Material::Dilithium),
            _ => Err("Unknown material string"),
        }
    }
}

// -----------------------------------------------------------------------------

// Materials stored as a bool array, in [generator, chip] pairs.
// Gen  location = material_id*2
// Chip location = material_id*2 + 1,
const MATERIAL_TYPES: usize = 9;
const FLOOR_SIZE: usize = MATERIAL_TYPES*2;

// -----------------------------------------------------------------------------

#[derive(Debug,Clone,Copy)]
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

    fn item_paired(&self, item: usize) -> bool {
        self.floors[self.elevator_idx].is_paired(item)
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

impl PartialEq for Building {
    fn eq(&self, other: &Building) -> bool {
        self.elevator_idx == other.elevator_idx &&
        self.floors[0] == other.floors[0] &&
        self.floors[1] == other.floors[1] &&
        self.floors[2] == other.floors[2] &&
        self.floors[3] == other.floors[3]
    }
}

impl Eq for Building {}

impl Hash for Building {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator_idx.hash(state);
        self.floors[0].hash(state);
        self.floors[1].hash(state);
        self.floors[2].hash(state);
        self.floors[3].hash(state);
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
const MAX_DEPTH: usize = 200;

// Returns the number of steps for everything to reach floor 4 (NO_PATH on failure)
fn process_bfs(initial: &Building, max_depth: usize) -> usize {

    let mut best_path = NO_PATH;
    let mut history: HashSet<Building> = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(*initial);

    while let Some(current) = queue.pop_front() {
        if current.depth > max_depth || current.depth > best_path {
            continue;
        }

        // Skip already visited states.
        // NOTE: if the current state has a shorter path then the previous
        // one we need to process it to ensure finding a shortest path.
        if let Some(previous_state) = history.replace(current) {
            if previous_state.depth <= current.depth {
                continue;
            }
        }

        // Check for goal state
        if current.is_final() {
            best_path = cmp::min(best_path, current.depth);
        }

        let mut can_move_single_down = false;
        for item_1 in 0..FLOOR_SIZE {
            if current.item_exists(item_1) {
                // Only try moving one item down
                if let Some(next) = current.try_move_down(item_1, item_1) {
                    queue.push_back(next);
                    can_move_single_down = true;
                }
            }
        }

        let mut first_pair_idx = None;
        for item_1 in 0..FLOOR_SIZE {
            if !current.item_exists(item_1) { continue; }

            // Only try moving one item down
            if let Some(next) = current.try_move_down(item_1, item_1) {
                queue.push_back(next);
                can_move_single_down = true;
            }

            if first_pair_idx.is_none() && current.item_paired(item_1) {
                first_pair_idx = Some(item_1);
            }

            // Skip moving paired items beyond the first pair.
            if let Some(idx)= first_pair_idx {
                if current.item_paired(item_1) && item_1 > idx+1 {
                    continue;
                }
            }

            for item_2 in item_1..FLOOR_SIZE {
                if !current.item_exists(item_2) { continue; }

                // Skip moving paired items beyond the first pair.
                if let Some(idx)= first_pair_idx {
                    if current.item_paired(item_2) && item_2 > idx+1 {
                        continue;
                    }
                }

                // Try moving both items up or down
                if let Some(next) = current.try_move_up(item_1, item_2) {
                    queue.push_back(next);
                }

                // Try moving two items down if it wasn't safe to move a single one.
                if !can_move_single_down && item_1 != item_2 {
                    if let Some(next) = current.try_move_down(item_1, item_2) {
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    println!("BFS terminated after searching {} unique nodes", history.len());

    best_path
}


fn read_input(input: &str) -> Building {
    let regex_chip = Regex::new("([:alpha:]+)-compatible").unwrap();
    let regex_gen = Regex::new("([:alpha:]+) generator").unwrap();

    let mut building = Building::new();
    for (floor_idx, line) in input.lines().enumerate() {

        for cap in regex_chip.captures_iter(line) {
            let chip_type = cap.at(1).unwrap().parse::<Material>().expect("Unknown chip material");
            //println!("Floor {}: {:?} type microchip", floor_idx, chip_type);
            building.floors[floor_idx].add_chip(chip_type);
        }

        for cap in regex_gen.captures_iter(line) {
            let gen_type = cap.at(1).unwrap().parse::<Material>().expect("Unknown gen material");
            //println!("Floor {}: {:?} type generator", floor_idx, gen_type);
            building.floors[floor_idx].add_gen(gen_type);
        }
    }
    assert!(building.is_safe());
    building
}

// -----------------------------------------------------------------------------

fn main() {
    // Part 1
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let building = read_input(&input_string);

    println!("Input building:\n{}", &building);

    println!("Searching for solution...");
    let steps = process_bfs(&building, MAX_DEPTH);

    if steps < NO_PATH {
        println!("Part 1: steps = {:?}", steps);
        assert!(steps == 47);
    } else {
        println!("Part 1: no path found within {} steps", MAX_DEPTH);
    }


    // ----------------------------------------------
    // Part 2
    let mut building2 = building;
    building2.floors[0].add_chip(Material::Elerium);
    building2.floors[0].add_gen(Material::Elerium);
    building2.floors[0].add_chip(Material::Dilithium);
    building2.floors[0].add_gen(Material::Dilithium);

    let steps2 = process_bfs(&building2, MAX_DEPTH);

    if steps2 < NO_PATH {
        println!("Part 2: steps = {:?}", steps2);
        assert!(steps2 > 47);
    } else {
        println!("Part 2: no path found within {} steps", MAX_DEPTH);
    }

}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_bfs() {
        let test_input = "The first floor contains a hydrogen-compatible microchip and a \
                      lithium-compatible microchip.\nThe second floor contains a hydrogen \
                      generator.\nThe third floor contains a lithium generator.\nThe fourth floor \
                      contains nothing relevant.\n";

        let building = read_input(&test_input);
        assert!(building.is_safe());

        //println!("Building:\n{}", &building);

        println!("Searching for solution...");
        let steps = process_bfs(&building, 11);

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

    #[test]
    fn test_hash() {
        let mut b1 = Building::new();
        b1.floors[1].add_gen(Material::Hydrogen);
        b1.floors[1].add_chip(Material::Hydrogen);

        let mut b2 = Building::new();
        b2.floors[1].add_gen(Material::Lithium);
        b2.floors[1].add_chip(Material::Lithium);
        //println!("Building:\n{}", &building);

        println!("b1 hash: {:?}", hasher(&b1));
        println!("b2 hash: {:?}", hasher(&b2));
        assert!(hasher(&b1) == hasher(&b2));

        b2.floors[0].add_chip(Material::Cobalt);
        assert!(hasher(&b1) != hasher(&b2));
    }

    fn hasher<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
