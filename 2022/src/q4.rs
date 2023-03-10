use std::io::BufRead;
use std::{fs, io::BufReader};

struct Range {
    pub lower_bound: u32,
    pub upper_bound: u32,
}

pub fn part_one() {
    q4(&|l: Range, r: Range| {
        let l_contains_r = l.lower_bound <= r.lower_bound && l.upper_bound >= r.upper_bound;
        let r_contains_l = l.lower_bound >= r.lower_bound && l.upper_bound <= r.upper_bound;
        l_contains_r || r_contains_l
    })
}

pub fn part_two() {
    q4(&|l: Range, r: Range| {
        let l_left_of_r = l.upper_bound < r.lower_bound;
        let l_right_of_r = l.lower_bound > r.upper_bound;
        !(l_left_of_r || l_right_of_r)
    })
}

fn q4(filter_pred: &dyn Fn(Range, Range) -> bool) {
    let file = fs::File::open("resources/q4").map(BufReader::new);
    let result = file.map(|reader| {
        reader
            .lines()
            .map(Result::unwrap)
            .filter(|line| {
                let mut split = line.split(',');

                let mut l_range = split.next().unwrap().split('-');
                let l = Range {
                    lower_bound: l_range.next().unwrap().parse().unwrap(),
                    upper_bound: l_range.next().unwrap().parse().unwrap(),
                };

                let mut r_range = split.next().unwrap().split('-');
                let r = Range {
                    lower_bound: r_range.next().unwrap().parse().unwrap(),
                    upper_bound: r_range.next().unwrap().parse().unwrap(),
                };

                filter_pred(l, r)
            })
            .collect::<Vec<_>>()
            .len()
    });

    match result {
        Err(err) => panic!("{err}"),
        Ok(len) => println!("Number of pairs: {len}"),
    }
}
