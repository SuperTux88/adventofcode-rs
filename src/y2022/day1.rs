use std::io::{self, BufRead};

use crate::aoc::Day;

pub struct Solution {
    elves_calories: Vec<u32>,
}

impl Day for Solution {
    fn title() -> &'static str {
        "Calorie Counting"
    }

    fn with_input(input: &mut impl BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let elves = input
            .split("\n\n")
            .map(|elf| elf.lines().map(|c| c.parse::<u32>().unwrap()));
        let mut elves_calories: Vec<u32> = elves.map(|elf| elf.sum()).collect();
        elves_calories.sort();

        Self { elves_calories }
    }

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
        let day = Solution::with_input(input!(example: 2022 1));
        assert_eq!(day.part1(), "24000");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 1));
        assert_eq!(day.part1(), "68442");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 1));
        assert_eq!(day.part2(), "45000");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 1));
        assert_eq!(day.part2(), "204837");
    }
}
