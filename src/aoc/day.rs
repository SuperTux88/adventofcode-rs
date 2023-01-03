use std::io::BufRead;

pub trait Day {
    fn title() -> &'static str;

    fn with_input(input: &mut impl BufRead) -> Self;

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
