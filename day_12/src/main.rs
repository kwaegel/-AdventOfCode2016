
use std::fs::File;
use std::io::Read;
use std::str::FromStr;


#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
impl FromStr for Register {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Register, &'static str> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err("Unknown register string"),
        }
    }
}

// -----------------------------------------------------------------------------

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Source {
    Reg (Register),
    Val (i32),
}
impl FromStr for Source {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Source, &'static str> {

        //s.parse::<Register>().or_else(Source::Val(s.parse::<i32>())?)

        if let Ok(reg) = s.parse::<Register>() {
            Ok(Source::Reg(reg))
        } else {
            s.parse::<i32>().map(|i| Source::Val(i))
                .map_err(|_| "Failed to parse int")
        }
    }
}

// -----------------------------------------------------------------------------

// cpy x y copies x (either an integer or the value of a register) into register y.
// inc x increases the value of register x by one.
// dec x decreases the value of register x by one.
// jnz x y jumps to an instruction y away (positive means forward; negative means backward),
//      but only if x is not zero.
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Instruction {
    Cpy {src: Source, dst: Register},
    Inc {dst: Register},
    Dec {dst: Register},
    Jnz {src: Source, offset: i32},
}
impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Instruction, &'static str> {
        let tokens: Vec<_> = s.split(' ').collect();
        match tokens[0] {
            "cpy" => Ok(Instruction::Cpy{ src: tokens[1].parse()?, dst: tokens[2].parse()? }),
            "inc" => Ok(Instruction::Inc{dst: tokens[1].parse()?}),
            "dec" => Ok(Instruction::Dec{dst: tokens[1].parse()?}),
            "jnz" => Ok(Instruction::Jnz{
                src: tokens[1].parse()?,
                offset: tokens[2].parse().expect("Failed to parse int")
            }),
            _ => Err("Unable to parse instruction"),
        }
    }
}

// -----------------------------------------------------------------------------

fn run_until_halt(instructions: &Vec<Instruction>, starting_registers: [i32; 4]) -> [i32; 4] {

    let mut pc = 0;
    let mut registers = starting_registers;

    while pc < instructions.len() {
        //std::thread::sleep_ms(500);
        let ins = instructions[pc];
        //println!("{},  {:32?},  {:?}", pc, ins, registers);
        match ins {
            Instruction::Cpy{src, dst} => {
                match src {
                    Source::Reg(src) => {registers[dst as usize] = registers[src as usize]},
                    Source::Val(i) => {registers[dst as usize] = i},
                };
                pc += 1;
            },
            Instruction::Inc{dst} => {registers[dst as usize] += 1; pc += 1;},
            Instruction::Dec{dst} => {registers[dst as usize] -= 1; pc += 1;},
            Instruction::Jnz{src, offset} => {
                let src_val = match src {
                    Source::Reg(src) => {registers[src as usize]},
                    Source::Val(i) => i,
                };
                if src_val != 0 {
                    let new_pc = pc as i32 + offset;
                    pc = new_pc as usize;
                } else {
                    pc += 1;
                }
            },
        }
    }
    registers
}

// -----------------------------------------------------------------------------

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    let instructions: Vec<_> = input_string
        .lines()
        .map(|line| line.parse::<Instruction>().expect("parse failure"))
        .collect();


    {
        let mut registers1 = [0i32; 4];
        registers1 = run_until_halt(&instructions, registers1);

        println!("Part 1: registers: {:?}", registers1);
        assert!(registers1[Register::A as usize] == 318007);
    }

    {
        // part 2
        let mut registers2 = [0, 0, 1, 0];
        registers2 = run_until_halt(&instructions, registers2);

        println!("Part 2: registers: {:?}", registers2);
        assert!(registers2[Register::A as usize] == 9227661);
    }
}
