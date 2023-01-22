use std::io::{self, BufRead};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "TODO";

pub struct Solution {}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();

        todo!();

        Self {}
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        todo!();
    }

    fn part2(&self) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "");
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "");
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "");
    }
}
