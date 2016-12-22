
use std::collections::VecDeque;

//struct Point {
//    x: usize,
//    y: usize,
//    path_length: u32,
//    euclidean_dist: f32,
//}
//fn dist(p1: Point, p2: Point) -> f32 {
//    let dx = p2.x as f32 - p1.x as f32;
//    let dy = p2.x as f32 - p1.x as f32;
//    (dx*dx + dy*dy).sqrt()
//}

// Find x*x + 3*x + 2*x*y + y + y*y.
// Add the office designer's favorite number (your puzzle input).
// Find the binary representation of that sum; count the number of bits that are 1.
// If the number of bits that are 1 is even, it's an open space.
// If the number of bits that are 1 is odd, it's a wall.
fn is_open(target: (i32,i32), magic_number: i32) -> bool {

    let (x,y) = target;
    let set_bits = ((x*x + 3*x + 2*x*y + y + y*y) + magic_number).count_ones();

    let is_even = |x: u32| x & 1 == 0;
    if is_even(set_bits) {true} else {false}
}

fn in_bounds((x,y): (i32, i32), size: i32) -> bool {
    x >= 0 && x < size && y >= 0 && y < size
}


fn search(target: (i32,i32), magic_number: i32) -> Option<i32> {
    let start = (1_i32, 1_i32, 0_i32);
    const SIZE: i32 = 100;
    let mut visited = [[false; SIZE as usize]; SIZE as usize]; // Each cell stores [visited] state.

    // Try a simple BFS search
    let mut shortest_path = SIZE*SIZE;
    let mut queue = VecDeque::new();
    queue.push_back(start); // x,y,path

    while let Some(next) = queue.pop_front() {
        //println!("Checking {:?}", next);
        let pos = (next.0, next.1);
        if in_bounds(pos, SIZE)
            && is_open(pos, magic_number)
            && !visited[pos.0 as usize][pos.1 as usize]
            && next.2 <shortest_path {

            visited[pos.0 as usize][pos.1 as usize] = true;

            if pos == target && next.2 < shortest_path {
                println!("Found new path to target with distance {}", next.2);
                shortest_path = next.2;
            } else {
                let x = next.0;
                let y = next.1;
                let dist = next.2;
                // Generate next states
                queue.push_back((x+1, y, dist+1));
                queue.push_back((x-1, y, dist+1));
                queue.push_back((x, y+1, dist+1));
                queue.push_back((x, y-1, dist+1));
            }
        } // else skip state
    }

    if shortest_path < (SIZE*SIZE) {
        Some(shortest_path)
    } else {
        None
    }
}


fn max_within_range(max_path: i32,magic_number: i32) -> i32 {
    let start = (1_i32, 1_i32, 0_i32);
    const SIZE: i32 = 100;
    let mut visited = [[false; SIZE as usize]; SIZE as usize]; // Each cell stores [visited] state.

    // Try a simple BFS search
    let mut visited_states = 0i32;
    let mut queue = VecDeque::new();
    queue.push_back(start); // x,y,path

    while let Some(next) = queue.pop_front() {
        let pos = (next.0, next.1);
        if in_bounds(pos, SIZE)
            && is_open(pos, magic_number)
            && !visited[pos.0 as usize][pos.1 as usize]
            && next.2 <= max_path {

                visited[pos.0 as usize][pos.1 as usize] = true;
                visited_states += 1;

                let x = next.0;
                let y = next.1;
                let dist = next.2;
                // Generate next states
                queue.push_back((x+1, y, dist+1));
                queue.push_back((x-1, y, dist+1));
                queue.push_back((x, y+1, dist+1));
                queue.push_back((x, y-1, dist+1));
        }
    } // else skip state
    visited_states
}


fn main() {
    println!("Hello, world!");

    let puzzle_magic_number = 1362_i32;
    let target = (31,39); // (x,y)

    let path = search(target, puzzle_magic_number);
    println!("Part 1: path distance = {:?}", path);
    assert!(path == Some(82));

    let visited_states = max_within_range(50, puzzle_magic_number);
    println!("Part 2: possible to visit {} states", visited_states);
    assert!(visited_states == 138);

//    let example_magic = 10;
//    for y in 0..7 {
//        for x in 0..10 {
//            let open = is_open((x,y), example_magic);
//            print!("{}", if open {"."} else {"#"});
//        }
//        println!("");
//    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_open() {
        assert!(is_open((1,1), 10));
    }

    #[test]
    fn test() {
        let dist = search((7,4), 10);
        println!("Test distance = {:?}", dist);
        assert!(dist == Some(11));
    }
}
