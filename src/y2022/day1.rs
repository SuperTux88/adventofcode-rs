use std::io::{self, BufRead};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Calorie Counting";

pub struct Solution {
    elves_calories: Vec<u32>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let elves = input
            .split("\n\n")
            .map(|elf| elf.lines().map(|c| c.parse::<u32>().unwrap()));
        let mut elves_calories: Vec<u32> = elves.map(|elf| elf.sum()).collect();
        elves_calories.sort();

        Self { elves_calories }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.elves_calories.last().unwrap().to_string()
    }

    fn part2(&self) -> String {
        self.elves_calories
            .iter()
            .rev()
            .take(3)
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "24000");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "68442");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "45000");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "204837");
    }
}
