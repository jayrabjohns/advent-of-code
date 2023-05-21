use std::{error::Error, fs};

use super::Question;

pub struct Q2;
impl Question for Q2 {
    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        q2(&calc_score_p1)
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        q2(&calc_score_p2)
    }
}

fn q2(calc_score: &dyn Fn(&str) -> u32) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string("resources/q2")?;
    let score = contents.lines().map(|round| calc_score(round)).sum::<u32>();
    Ok(score.to_string())
}

fn calc_score_p1(round: &str) -> u32 {
    match round {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        undefined => panic!("Cannot parse invalid round `{undefined}`."),
    }
}

fn calc_score_p2(round: &str) -> u32 {
    match round {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        undefined => panic!("Cannot parse invalid round `{undefined}`."),
    }
}
