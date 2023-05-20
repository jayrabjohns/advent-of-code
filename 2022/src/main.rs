mod questions;

use questions::{Question, Q1, Q2, Q3, Q4, Q5, Q6};
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !validate_args(&args) {
        print_usage();
        return;
    }

    let input = args[1].as_str();
    let questions: Vec<(&str, Box<dyn Question>)> = vec![
        ("q1", Box::new(Q1)),
        ("q2", Box::new(Q2)),
        ("q3", Box::new(Q3)),
        ("q4", Box::new(Q4)),
        ("q5", Box::new(Q5)),
        ("q6", Box::new(Q6)),
    ];
    questions
        .iter()
        .find(|(q_str, _)| q_str == &input)
        .map_or_else(
            || {
                println!(
                    "Invalid question number. There are {} questions",
                    questions.len()
                );
            },
            |(_, question)| question.run_all(),
        );
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
