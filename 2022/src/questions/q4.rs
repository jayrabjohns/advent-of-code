use std::{error::Error, fs};

use super::Question;

struct Range {
    pub lower_bound: u32,
    pub upper_bound: u32,
}

pub struct Q4;
impl Question for Q4 {
    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        q4(&|l: &Range, r: &Range| {
            let l_contains_r = l.lower_bound <= r.lower_bound && l.upper_bound >= r.upper_bound;
            let r_contains_l = l.lower_bound >= r.lower_bound && l.upper_bound <= r.upper_bound;
            l_contains_r || r_contains_l
        })
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        q4(&|l: &Range, r: &Range| {
            let l_left_of_r = l.upper_bound < r.lower_bound;
            let l_right_of_r = l.lower_bound > r.upper_bound;
            !(l_left_of_r || l_right_of_r)
        })
    }
}

fn q4(filter_pred: &dyn Fn(&Range, &Range) -> bool) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string("resources/q4")?;
    let result: Result<Vec<(Range, Range)>, Box<dyn Error>> = contents
        .lines()
        .flat_map(|line| {
            let mut split = line.split(',');

            let mut lhs = split.next()?.split('-');
            let mut rhs = split.next()?.split('-');

            let l_range = split_to_range(lhs.next()?, lhs.next()?);
            let r_range = split_to_range(rhs.next()?, rhs.next()?);

            Some(l_range.and_then(|l| r_range.map(|r| (l, r))))
        })
        .collect();

    let filtered_results = result?.into_iter().filter(|(l, r)| filter_pred(l, r));

    Ok(filtered_results.count().to_string())
}

fn split_to_range(lower_bound_str: &str, upper_bound_str: &str) -> Result<Range, Box<dyn Error>> {
    Ok(Range {
        lower_bound: lower_bound_str.parse()?,
        upper_bound: upper_bound_str.parse()?,
    })
}
