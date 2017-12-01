


fn steal(num_elves: usize) -> usize {

    let mut presents = Vec::new();
    presents.resize(num_elves, 1);

    let mut thief_idx = 0;
    loop {

        // Advance thief index
        while presents[thief_idx] == 0 {
            thief_idx = (thief_idx + 1) % num_elves;
        }

        // Find the next victim with >0 presents
        let mut victim_idx = (thief_idx + 1) % num_elves;
        while presents[victim_idx] == 0 {
            victim_idx = (victim_idx + 1) % num_elves;
        }

        // Transfer
        presents[thief_idx] += presents[victim_idx];
        presents[victim_idx] = 0;

        // If one elf has all the presents, return that index
        if presents[thief_idx] == num_elves {
            return thief_idx;
        }

        // Set the next potential thief
        thief_idx = (victim_idx+1) % num_elves;
    }
}


fn main() {

    let idx = steal(5);
    println!("Ring size 5: winner is elf {}", idx+1);

    let part1_idx = steal(3005290);
    println!("Part 1: ring size 3005290 = winner is elf {}", part1_idx+1);
}
