use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

use substring::Substring;

pub fn part_one() {
    let file = std::fs::File::open("resources/q3").map(|file| BufReader::new(file));
    let result = file.map(|reader| {
        reader
            .lines()
            .map(|line| line.map(calc_score))
            .collect::<Result<Vec<_>, _>>()
            .map(|vec| vec.iter().sum::<u32>())
    });

    match result {
        Err(err) => panic!("{err}"),
        Ok(Err(err)) => panic!("{err}"),
        Ok(Ok(sum)) => println!("Sum of priorities: {sum}"),
    }
}
pub fn part_two() {}

fn calc_score(line: String) -> u32 {
    let mid_point = line.len() / 2;
    let left: &str = line.substring(0, mid_point);
    let right: &str = line.substring(mid_point, line.len());
    let left_set: HashSet<char> = left.chars().collect();
    return right
        .chars()
        .filter(|c| left_set.contains(&c))
        .next()
        .map(get_priority)
        .unwrap()
        .unwrap();
}

fn get_priority(char: char) -> Option<u32> {
    if !char.is_ascii() {
        return None;
    }

    let priority = if char.is_uppercase() {
        (char as u32) - ('A' as u32) + 26
    } else {
        (char as u32) - ('a' as u32)
    } + 1;

    //println!("Priority for {char} = {priority}");

    return Some(priority);
}
