use regex::Regex;
use std::fs;

use super::Question;

type CrateStack = Vec<char>;
type Move = [usize; 3];

pub struct Q5;
impl Question for Q5 {
    fn part_one(&self) {
        q5(
            &|move_count: usize, start: usize, finish: usize, stacks: &mut Vec<CrateStack>| {
                for _ in 0..move_count {
                    let elem = stacks[start].pop().expect("Empty Stack");
                    // println!("Moving {elem} from stack {} to {}", start + 1, finish + 1);
                    stacks[finish].push(elem);
                }
            },
        )
    }

    fn part_two(&self) {
        q5(
            &|move_count: usize, start: usize, finish: usize, stacks: &mut Vec<CrateStack>| {
                let len = stacks[start].len();
                let elems = &mut stacks[start]
                    .drain((len - move_count)..)
                    .collect::<Vec<_>>();
                stacks[finish].append(elems);
            },
        )
    }
}

fn q5(move_fn: &dyn Fn(usize, usize, usize, &mut Vec<CrateStack>) -> ()) {
    let file_contents = fs::read_to_string("resources/q5").unwrap();
    let (starting_state, moves) = file_contents.split_once("\n\n").unwrap();
    let mut stacks: Vec<CrateStack> = parse_start_state(starting_state);
    let moves: Vec<Move> = parse_moves(moves);

    for m in &moves {
        let move_count = m[0];
        let start = m[1] - 1;
        let finish = m[2] - 1;

        move_fn(move_count, start, finish, &mut stacks);
    }

    let mut result = String::new();
    for stack in stacks {
        let char = stack.last().expect("Empty stack");
        result.push(*char);
    }

    println!("Result: {result}");
}

fn parse_start_state(start_state: &str) -> Vec<CrateStack> {
    let mut lines = start_state.lines().map(|line| line.chars()).rev();

    let last_line = lines.next().unwrap();
    let num_stacks = last_line
        .filter(|c| c.is_numeric())
        .last()
        .and_then(|c| c.to_digit(10))
        .expect("Expected integer");

    let lines = lines.map(|x| x.collect::<Vec<_>>());
    let mut stacks: Vec<CrateStack> = vec![vec![]; num_stacks as usize];
    for line in lines {
        //println!("-> `{}`", line.iter().collect::<String>());
        for (i, char) in line.iter().skip(1).step_by(2).enumerate() {
            if !char.is_whitespace() {
                stacks[i / 2].push(*char);
            }
        }
    }

    return stacks;
}

fn parse_moves(moves: &str) -> Vec<Move> {
    let int_regex = Regex::new(r"\d+").unwrap();
    let moves = moves
        .lines()
        .map(|line| {
            let mut ints = int_regex
                .find_iter(line)
                .filter_map(|m| m.as_str().parse::<usize>().ok());
            [
                ints.next().unwrap(),
                ints.next().unwrap(),
                ints.next().unwrap(),
            ]
        })
        .collect();

    return moves;
}
