
use std::io::prelude::*;
use std::fs::File;

use std::ops::Add;
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

// North = +y
// East = +X
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2i {
    x: i32,
    y: i32,
}
impl Vec2i {
    fn new (x: i32, y:i32) -> Vec2i {
        Vec2i{x:x, y:y}
    }
    // Component-wise multiplication
    fn mult(v1: Vec2i, scalar: i32) -> Vec2i {
        let x = v1.x*scalar;
        let y = v1.y*scalar;
        Vec2i{x:x, y: y}
    }

    // Distance from origin
    fn manhattan_magnitude(&self) -> i32 {
        &self.x.abs() + &self.y.abs()
    }
}
impl Add for Vec2i {
    type Output = Vec2i;
    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Pose {
    direction: Vec2i,
    position: Vec2i,
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Right,
    Left,
    Straight
}

// Given a input direction vector (x,y), rotate θ° CCW using the rotation matrix:
// R = [cos θ, -sin θ]
//     [sin θ,  cos θ]
//
// Left: (90°) = [0 -1][x]
//               [1  0][y]
// Right: (-90°) = [ 0 1][x]
//                 [-1 0][y]
fn rotated(input: Vec2i, turn: Turn) -> Vec2i {
    match turn {
        Turn::Left => Vec2i::new(-input.y, input.x),
        Turn::Right => Vec2i::new(input.y, -input.x),
        Turn::Straight => input,
    }
}

// (direction, position)
fn accumulate(current: Pose, turn: Turn, dist: i32) -> Pose {

    let new_direction= rotated(current.direction, turn);
    let new_position = current.position + Vec2i::mult(new_direction, dist);
    Pose{direction: new_direction, position: new_position}
}

fn as_direction(input: Option<&str>) -> Turn {
    match input {
        Some("R") => Turn::Right,
        _ => Turn::Left
    }
}

fn get_final_pose(input_string: &str) -> Pose {
    let re = Regex::new(r"([RL])([:digit:]+)").unwrap();
    let mut pose = Pose{position: Vec2i::new(0,0), direction:  Vec2i::new(0,1)};
    for cap in re.captures_iter(&input_string) {
        let turn = as_direction(cap.at(1));
        let dist: i32 = cap.at(2).unwrap().parse().unwrap();
        pose = accumulate(pose, turn, dist);
    }
    pose
}

fn get_first_crossing(input_string: &str) -> Pose {
    let re = Regex::new(r"([RL])([:digit:]+)").unwrap();
    let mut pose = Pose{position: Vec2i::new(0,0), direction:  Vec2i::new(0,1)};
    let mut history = HashSet::new();

    'outer: for cap in re.captures_iter(&input_string) {
        let turn = as_direction(cap.at(1));
        let dist: i32 = cap.at(2).unwrap().parse().unwrap();

        // Rotate without moving
        pose.direction = rotated(pose.direction, turn);

        // Accumulate <dist> steps of size 1
        for _ in 0..dist {
            pose = accumulate(pose, Turn::Straight, 1);

            // Keep a history of positions
            if history.contains(&pose.position) {
                break 'outer;
            }
            history.insert(pose.position);
        }
    }
    pose
}

fn main() {

    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    // Part 1
    {
        let pose = get_final_pose(&input_string);
        println!("Part 1: stopped at {:?}, distance {}",
                 pose.position, pose.position.manhattan_magnitude());
    }

    // Part 2
    {
        let pose = get_first_crossing(&input_string);
        println!("Part 2: stopped at {:?}, distance {}",
                 pose.position, pose.position.manhattan_magnitude()); // 257 is too high. 105 is too low.
    }
}

//-----------------------------------------------------------------------------

#[test]
fn test1() {
    let pose = get_final_pose("R2, L3");
    //println!("{:?}", pose);
    assert!(pose.position.x == 2);
    assert!(pose.position.y == 3);
    assert!(pose.position.manhattan_magnitude() == 5);
}

#[test]
fn test2() {
    let pose = get_final_pose("R2, R2, R2");
    //println!("{:?}", pose);
    assert!(pose.position.x == 0);
    assert!(pose.position.y == -2);
}

#[test]
fn test3() {
    let pose = get_final_pose("R5, L5, R5, R3");
    //println!("{:?}", pose);
    assert!(pose.position.manhattan_magnitude() == 12);
}

#[test]
fn revisit1() {
    let pose = get_first_crossing("R8, R4, R4, R8");

    println!("revisit1: stopped at {:?}, distance {}",
             pose.position, pose.position.manhattan_magnitude()); // 257 is too high.
    assert!(pose.position.manhattan_magnitude() == 4);
}