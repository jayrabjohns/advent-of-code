use std::error::Error;

pub trait Question {
    fn part_one(&self) -> Result<String, Box<dyn Error>>;
    fn part_two(&self) -> Result<String, Box<dyn Error>>;

    fn run_all(&self) {
        println!("=== Part 1 ===");
        print_result(self.part_one());

        println!("=== Part 2 ===");
        print_result(self.part_two());
    }
}

fn print_result(result: Result<String, Box<dyn Error>>) {
    match result {
        Ok(result) => println!("Result: {result}"),
        Err(error) => panic!("{error}"),
    }
}
