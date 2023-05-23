use std::{collections::HashSet, error::Error, fs, io};

use super::Question;

pub struct Q3;
impl Question for Q3 {
    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string("resources/q3")?;
        let priorities: Option<Vec<u32>> = contents
            .lines()
            .map(&split_string_in_half)
            .map(|(left, right)| {
                let left_set: HashSet<char> = left.chars().collect();
                right
                    .chars()
                    .filter(|c| left_set.contains(c))
                    .next()
                    .map(|char| get_priority(&char))
            })
            .collect();

        match priorities {
            Some(priorities) => Ok(priorities.iter().sum::<u32>().to_string()),
            None => Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "Empty file",
            ))),
        }
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string("resources/q3")?;
        let lines: Vec<&str> = contents.lines().collect();
        let sum_priorities: u32 = lines
            .chunks(3)
            .map(&find_common_char)
            .flat_map(|chars| chars.get(0).map(&get_priority))
            .sum();

        Ok(sum_priorities.to_string())
    }
}

fn find_common_char<'a>(chunk: &[&str]) -> Vec<char> {
    let mut chunk = chunk.iter();
    chunk
        .next()
        .map(|&line| line.chars().collect::<Vec<_>>())
        .map(|mut possibilities| {
            chunk.for_each(|&line| {
                possibilities = line
                    .chars()
                    .filter(|char| possibilities.contains(char))
                    .collect()
            });
            possibilities
        })
        .unwrap_or(vec![])
}

fn split_string_in_half<'a>(str: &'a str) -> (&'a str, &'a str) {
    let mid_point = str.len() / 2;
    (&str[0..mid_point], &str[mid_point..])
}

fn get_priority(char: &char) -> u32 {
    if !char.is_ascii_alphabetic() {
        return 0;
    }

    (if char.is_uppercase() {
        (*char as u32) - ('A' as u32) + 26
    } else {
        (*char as u32) - ('a' as u32)
    }) + 1
}
