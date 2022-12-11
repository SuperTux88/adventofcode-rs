use std::io::BufRead;

pub trait AoCDay {
    fn with_input(input: &mut impl BufRead) -> Self;

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub fn run_day(day: impl AoCDay) {
    println!("Part 1: {}", day.part1());
    println!("Part 2: {}", day.part2());
}
