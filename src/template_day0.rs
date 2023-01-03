use std::io::{self, BufRead};

use crate::aoc::Day;

pub struct Solution {}

impl Day for Solution {
    fn title() -> &'static str {
        "TODO"
    }

    fn with_input(input: &mut impl BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();

        todo!();

        Self {}
    }

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
        let day = Solution::with_input(input!(example: 2022 0));
        assert_eq!(day.part1(), "");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 0));
        assert_eq!(day.part1(), "");
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 0));
        assert_eq!(day.part2(), "");
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 0));
        assert_eq!(day.part2(), "");
    }
}
