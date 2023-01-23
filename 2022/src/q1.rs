use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("../resources/q1");
    let result = input.map(|contents| {
        contents
            .split("\n\n")
            .map(|chunk| {
                chunk
                    .lines()
                    .filter_map(|line| line.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
    });
    match result {
        Err(err) => println!("{:?}", err),
        Ok(None) => println!("Empty file"),
        Ok(Some(maximum)) => println!("Max calories: {}", maximum),
    };
}

fn part_two() {}
