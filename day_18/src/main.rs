
fn get_checked(row: &Vec<char>, index: i32) -> char {
    if index < 0 || index >= row.len() as i32 {
        '.'
    } else {
        row[index as usize]
    }
}

fn count_safe(first_row: &str, num_rows: i32) -> usize {
    let mut row = first_row.chars().collect::<Vec<_>>();

    let mut num_safe = row.iter().filter(|&c| c == &'.').count();

    println!("{}", row.iter().cloned().collect::<String>());
    for _ in 1..num_rows {
        let mut next_row = Vec::new();
        for idx in 0..row.len() {
            let i = idx as i32;
            //let left = if (i == 0 || i == row.len()-1) {&'.'} else {row.get(i-1)};
            //let left = row.get(i - 1).unwrap_or(&'.') == &'^';
            let left = get_checked(&row, i-1) == '^';
            let center = get_checked(&row, i) == '^';
            let right = get_checked(&row, i+1) == '^';

            let is_trap =
                (left && center && !right)
                    || (center && right && !left)
                    || (left && !center && !right)
                    || (right && !left && !center);

            next_row.push(if is_trap {'^'} else {'.'});
        }
        row = next_row;
        num_safe += row.iter().filter(|&c| c == &'.').count();
        println!("{}", row.iter().cloned().collect::<String>());
    }

    num_safe
}

fn main() {
    let safe_count1 = count_safe("..^^.", 3);
    println!("Example 1: {} safe tiles", safe_count1);

    let safe_count2 = count_safe(".^^.^.^^^^", 10);
    println!("Example 2: {} safe tiles", safe_count2);

    let part1_input = "^.^^^.^..^....^^....^^^^.^^.^...^^.^.^^.^^.^^..^.^...^.^..^.^^.^..^.....^^^.^.^^^..^^...^^^...^...^.";
    let part1_safe = count_safe(part1_input, 40);
    println!("Part 1: {} safe tiles", part1_safe);
}
