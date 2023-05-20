use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

use super::Question;

pub struct Q3;
impl Question for Q3 {
    fn part_one(&self) {
        let file = std::fs::File::open("resources/q3").map(|file| BufReader::new(file));
        let result = file.map(|reader| {
            reader
                .lines()
                .map(|line| line.map(calc_score_p1))
                .collect::<Result<Vec<_>, _>>()
                .map(|vec| vec.iter().sum::<u32>())
        });

        match result {
            Err(err) => panic!("{err}"),
            Ok(Err(err)) => panic!("{err}"),
            Ok(Ok(sum)) => println!("Sum of priorities: {sum}"),
        }
    }

    fn part_two(&self) {
        let file = std::fs::File::open("resources/q3").map(|file| BufReader::new(file));
        let result: Result<u32, io::Error> = file.map(|reader| {
            reader
                .lines()
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|chunk| {
                    let chunk = chunk.iter().map(|line| match line {
                        Err(err) => panic!("{err}"),
                        Ok(str) => str,
                    });
                    find_common_char(chunk)
                        .map(|possibilites| get_priority(possibilites[0]))
                        .unwrap()
                        .unwrap()
                })
                .sum()
        });

        match result {
            Err(err) => panic!("{err}"),
            Ok(sum) => println!("Sum of priorities: {sum}"),
        }
    }
}

fn find_common_char<'a>(chunk: impl Iterator<Item = &'a String>) -> Option<Vec<char>> {
    let mut chunk = chunk;
    let mut possibilities: Option<Vec<char>> = chunk.next().map(|line| line.chars().collect());
    chunk.for_each(|line| {
        possibilities = possibilities
            .as_ref()
            .map(|chars| line.chars().filter(|char| chars.contains(char)).collect());
    });

    return possibilities;
}

fn calc_score_p1(line: String) -> u32 {
    let mid_point = line.len() / 2;
    let left = &line[0..mid_point];
    let right = &line[mid_point..];
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
    if !char.is_ascii_alphabetic() {
        return None;
    }

    let priority = if char.is_uppercase() {
        (char as u32) - ('A' as u32) + 26
    } else {
        (char as u32) - ('a' as u32)
    } + 1;

    // println!("Priority for {char} = {priority}");

    return Some(priority);
}
