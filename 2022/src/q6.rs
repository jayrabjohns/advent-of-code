use std::{collections::HashSet, fs};

pub fn part_one() {
    q6(4);
}

pub fn part_two() {
    q6(14);
}

fn q6(num_distinct_chars: usize) {
    let input = fs::read_to_string("resources/q6").unwrap();
    let mut start_index = 0;
    let mut end_index = num_distinct_chars - 1;
    let mut set = HashSet::<char>::new();

    while set.len() != num_distinct_chars {
        let slice = &input[start_index..=end_index];
        set.clear();
        for char in slice.chars() {
            set.insert(char);
        }

        end_index += 1;
        start_index += 1;
    }

    println!("First marker after character {end_index}");
}
