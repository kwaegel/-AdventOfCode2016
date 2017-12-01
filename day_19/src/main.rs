
use std::collections::VecDeque;


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

fn steal_across(initial_elves: usize) -> usize {

    // Note: A deque is a *terrible* data structure for this problem.
    // A Linked List or boost-like Deque would be *much* faster.
    let mut elf_ids: VecDeque<_> = (0..initial_elves).collect();

    let mut thief_idx = 0;
    while elf_ids.len() > 1 {

        // Identify the victim across the circle
        let half_size = elf_ids.len()/2;
        let victim_idx = (thief_idx + half_size) % elf_ids.len();

        // Transfer
        elf_ids.remove(victim_idx);

        // Relocate the thief, which is now off by one.
        if thief_idx > victim_idx {
            thief_idx = thief_idx - 1 % elf_ids.len();
        } else {
            thief_idx = thief_idx % elf_ids.len();
        }

        // Advance the thief index
        thief_idx = (thief_idx+1) % elf_ids.len();

        if elf_ids.len() % 1000 == 0 {
            println!("{}", elf_ids.len());
        }
    }

    elf_ids[0]
}


fn main() {

    let idx = steal(5);
    println!("Ring size 5: winner is elf {}", idx+1);

    let part1_idx = steal(3005290);
    println!("Part 1: ring size 3005290 = winner is elf {}", part1_idx+1);
    assert_eq!(part1_idx+1, 1816277);


    let across_idx = steal_across(5);
    println!("Part 2: Ring size 5 = winner is elf {}", across_idx+1);

    let part2_idx = steal_across(3005290);
    println!("Part 2: ring size 3005290 = winner is elf {}", part2_idx+1);
    assert_eq!(part2_idx+1, 1410967);
}
