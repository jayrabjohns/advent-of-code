mod questions;

use clap::Parser;
use questions::{Question, Q1, Q2, Q3, Q4, Q5, Q6 /* , Q5, Q6*/};

#[derive(Parser)]
struct Args {
    /// Day number - int in range [1,25]
    day: usize,
}

fn main() {
    let args = Args::parse();
    let questions: Vec<Box<dyn Question>> = vec![
        Box::new(Q1),
        Box::new(Q2),
        Box::new(Q3),
        Box::new(Q4),
        Box::new(Q5),
        Box::new(Q6),
    ];

    questions.get(args.day - 1).map_or_else(
        || println!("Invalid day number. There are {} days", questions.len()),
        |question| {
            println!("Running solutions for day {}", args.day);
            question.run_all();
        },
    );
}
