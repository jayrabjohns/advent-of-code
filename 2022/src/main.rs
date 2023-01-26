use regex::Regex;
use std::env;

mod q1;
mod q2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !validate_args(&args) {
        print_usage();
        return;
    }

    let question = args[1].as_str();
    let qs = ["q1", "q2"].map(|s| String::from(s));

    match question {
        "q1" => {
            println!("=== Part 1 ===");
            q1::part_one();
            println!("=== Part 2 ===");
            q1::part_two()
        }
        "q2" => {
            println!("=== Part 1 ===");
            q2::part_one();
            println!("=== Part 2 ===");
            q2::part_two()
        }
        _ => {
            println!("Invalid question number. There are {} questions", qs.len());
            return;
        }
    }
}

fn validate_args(args: &Vec<String>) -> bool {
    let question_regex = Regex::new(r"^q\d+$").unwrap();
    if args.len() > 0 && question_regex.is_match(&args[1]) {
        return true;
    }

    return false;
}

fn print_usage() {
    println!("advent-of-code-2022 <question>");
    println!("| where <question> is of the form q<question number>.");
    println!("| E.g. advent-of-code-2022 q1");
    println!("| This will run part one and part two of the question (if one exists).");
}
