use std::{collections::HashSet, fs};

pub fn part_one() {
    let input = fs::read_to_string("resources/q6").unwrap();
    let mut start_index = 0;
    let mut end_index = 3;
    let mut set = HashSet::<char>::new();

    while set.len() != 4 {
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

pub fn part_two() {}
