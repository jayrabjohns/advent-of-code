use std::{error::Error, fs, io};

use super::Question;

pub struct Q1;
impl Question for Q1 {
    fn part_one(&self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string("resources/q1")?;
        let max = chunk_sum(&contents).max().ok_or(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Empty file",
        )))?;

        Ok(max.to_string())
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string("resources/q1")?;
        let mut result: Vec<u32> = chunk_sum(&contents).collect();
        result.sort_by(|a, b| b.cmp(a));

        let sum: u32 = result.iter().take(3).sum();
        Ok(sum.to_string())
    }
}

fn chunk_sum(contents: &String) -> impl Iterator<Item = u32> + '_ {
    contents.split("\n\n").map(|chunk| {
        chunk
            .lines()
            .filter_map(|line| line.parse::<u32>().ok())
            .sum()
    })
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn part_one() {}
// }
