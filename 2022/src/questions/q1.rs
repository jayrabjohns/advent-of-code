use std::fs;

use super::Question;

pub struct Q1;
impl Question for Q1 {
    fn part_one(&self) {
        let file = fs::read_to_string("resources/q1");
        let result = file.map(|contents| chunk_sum(&contents).max());

        match result {
            Err(err) => println!("{:?}", err),
            Ok(None) => println!("Empty file"),
            Ok(Some(maximum)) => println!("Max calories: {}", maximum),
        };
    }

    fn part_two(&self) {
        let file = fs::read_to_string("resources/q1");
        let result = file
            .map(|contents| chunk_sum(&contents).collect::<Vec<u32>>())
            .map(|mut vec| {
                vec.sort();
                return vec;
            })
            .map(|vec| vec.iter().rev().take(3).sum::<u32>());

        match result {
            Err(err) => println!("{:?}", err),
            Ok(max) => println!("Max calories: {}", max),
        };
    }
}

fn chunk_sum(contents: &String) -> impl Iterator<Item = u32> + '_ {
    return contents.split("\n\n").map(|chunk| {
        chunk
            .lines()
            .filter_map(|line| line.parse::<u32>().ok())
            .sum::<u32>()
    });
}

