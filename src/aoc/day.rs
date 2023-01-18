use std::io::BufRead;

pub trait DayParser {
    fn with_input(input: &mut dyn BufRead) -> Self;
}

pub trait Day {
    fn title() -> &'static str;

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
