use std::fs::File;
use std::io::Read;
use std::collections::hash_map::HashMap;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

#[derive(Debug,Copy,Clone)]
enum Dest {
    None,
    Bot(i32),
    Output(i32),
}

fn make_dest(name: &str, id: i32) -> Dest {
    if name == "bot" {
        Dest::Bot(id)
    } else {
        Dest::Output(id)
    }
}

#[derive(Debug,Clone)]
struct Bot {
    chip_low: i32,
    chip_high: i32,
    output_low: Dest,
    output_high: Dest,
}

impl Bot {
    fn new() -> Bot {
        Bot {
            chip_low: -1,
            chip_high: -1,
            output_low: Dest::None,
            output_high: Dest::None,
        }
    }

    fn add_chip(&mut self, id: i32) {
        if id > self.chip_high {
            self.chip_low = self.chip_high;
            self.chip_high = id;
        } else if self.chip_low < id {
            self.chip_low = id
        } else {
            panic!("Unable to add chip");
        }
    }
}

fn require_bot(bots: &mut Vec<Bot>, bot_id: i32) {
    if bot_id as usize > bots.len() {
        bots.resize(bot_id as usize + 1, Bot::new());
    }
}

// Find the bot ID required to process [low_id, high_id]
fn process(input: &str, low_id: i32, high_id: i32) -> (HashMap<i32, i32>, i32) {

    let mut target_bot = -1i32;

    let mut bots = Vec::new();
    bots.reserve(500);

    let regex_initial = Regex::new("value ([:digit:]+) goes to bot ([:digit:]+)").unwrap();

    let regex_bot = Regex::new("bot ([:digit:]+) gives low to ([:alpha:]+) ([:digit:]+) and high \
                                to ([:alpha:]+) ([:digit:]+)")
        .unwrap();

    // Parse hand-off rules and initial state
    for line in input.lines() {
        if let Some(cap) = regex_initial.captures(line) {
            let chip_id: i32 = cap.at(1).unwrap().parse().unwrap();
            let bot_id: i32 = cap.at(2).unwrap().parse().unwrap();

            require_bot(&mut bots, bot_id);
            bots[bot_id as usize].add_chip(chip_id);
        } else if let Some(cap) = regex_bot.captures(line) {
            let bot_id: i32 = cap.at(1).unwrap().parse().unwrap();

            let low_output_str = cap.at(2).unwrap();
            let low_output_id: i32 = cap.at(3).unwrap().parse().unwrap();
            let low_output = make_dest(low_output_str, low_output_id);

            let high_output_str = cap.at(4).unwrap();
            let high_output_id: i32 = cap.at(5).unwrap().parse().unwrap();
            let high_output = make_dest(high_output_str, high_output_id);

            require_bot(&mut bots, bot_id);
            let bot = &mut bots[bot_id as usize];
            bot.output_low = low_output;
            bot.output_high = high_output;
        }
    }

    let mut output: HashMap<i32, i32> = HashMap::new();

    // Process chip movement
    let mut had_update = true; // halt when there are no more bots with 2 chips
    while had_update {
        had_update = false;
        // Need to iterate over bot IDs to avoid borrowing the Vec in the outer loop.
        for i in 0..bots.len() {

            if bots[i].chip_low == low_id && bots[i].chip_high == high_id {
                target_bot = i as i32;
            }

            if bots[i].chip_low >= 0 && bots[i].chip_high >= 0 {
                had_update = true;
                match bots[i].output_low {
                    Dest::Bot(target_id) => {
                        let id = bots[i].chip_low;
                        bots[target_id as usize].add_chip(id);
                    }
                    Dest::Output(output_bin) => {
                        output.insert(output_bin, bots[i].chip_low);
                    }
                    Dest::None => panic!("Low output unspecified"),
                }
                match bots[i].output_high {
                    Dest::Bot(target_id) => {
                        let id = bots[i].chip_high;
                        bots[target_id as usize].add_chip(id);
                    }
                    Dest::Output(output_bin) => {
                        output.insert(output_bin, bots[i].chip_high);
                    }
                    Dest::None => panic!("High output unspecified"),
                }
                bots[i].chip_low = -1;
                bots[i].chip_high = -1;
            }
            if bots[i].chip_low >= 0 && bots[i].chip_high >= 0 {
                panic!("Did not update chip correctly");
            }
        }
    }

    (output, target_bot)
}

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let results = process(&input_string, 17, 61);
    println!("Part 1: Bot {} compares chips {} and {}", results.1, 17, 61);

    let val_0 = results.0.get(&0).unwrap();
    let val_1 = results.0.get(&1).unwrap();
    let val_2 = results.0.get(&2).unwrap();
    let product = val_0 * val_1 * val_2;
    println!("Part 2: Product of output[0,1,2] is {}", product);
}

#[test]
fn test() {
    let test_data = "value 5 goes to bot 2\n
    bot 2 gives low to bot 1 and high to bot 0\n
    \
                     value 3 goes to bot 1\n
    bot 1 gives low to output 1 and high to bot 0\n
    \
                     bot 0 gives low to output 2 and high to output 0\n
    value 2 goes to bot \
                     2\n";

    let results = process(test_data, 2, 5);
    println!("Bot {} compares chips {} and {}", results.1, 2, 5);
    assert!(results.1 == 2);
}
