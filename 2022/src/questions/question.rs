pub trait Question {
    fn part_one(&self);
    fn part_two(&self);

    fn run_all(&self) {
        println!("=== Part 1 ===");
        self.part_one();

        println!("=== Part 2 ===");
        self.part_two();
    }
}
