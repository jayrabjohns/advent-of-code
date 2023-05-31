use std::{
    error::Error,
    fs,
    iter::{from_fn, Peekable},
};

use super::Question;

pub struct Q7;
impl Question for Q7 {
    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string("resources/q7")?;
        Ok(String::from(""))
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

fn calcSize(
    lines: &mut Peekable<impl Iterator<Item = &'static str>>,
    totalSize: &mut usize,
) -> Result<usize, Box<dyn Error>> {
    let mut size: usize;
    while let Some(l) = lines.next() {
        match l {
            "$ cd .." => break,
            "$ ls" => {
                size = from_fn(|| lines.next_if(|i| !i.starts_with('$')))
                    .filter(|&l| !l.starts_with("dir "))
                    .flat_map(|l| {
                        l.split(' ').next().map(|x| x.parse::<usize>()?)
                        // .parse::<usize>()
                        // .unwrap()
                    })
                    .sum()
            }
            _ => size = calcSize(lines, totalSize)?,
        }
    }

    if size <= 100_000 {
        *totalSize += size;
    }
    Ok(size)
}
