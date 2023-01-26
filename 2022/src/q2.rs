use std::fs;

pub fn part_one() {
    let file = fs::read_to_string("resources/q2");
    let result = file.map(|contents| {
        contents
            .split("\n")
            .map(|round| calc_score(round))
            .sum::<u32>()
    });

    match result {
        Err(err) => panic!("{err}"),
        Ok(score) => println!("Total score {score}"),
    }
}

pub fn part_two() {}

fn calc_score(round: &str) -> u32 {
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
