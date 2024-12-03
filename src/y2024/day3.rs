use std::io::{self, BufRead};

use regex::Regex;

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Mull It Over";

const REGEX: &str = r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))";

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

pub struct Solution {
    instructions: Vec<Instruction>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let regex = Regex::new(REGEX).unwrap();
        let instructions = regex
            .captures_iter(input.as_str())
            .map(|cap| match &cap[1] {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Dont,
                _ => Instruction::Mul(cap[2].parse().unwrap(), cap[3].parse().unwrap()),
            })
            .collect();

        Self { instructions }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.instructions
            .iter()
            .fold(0, |acc, inst| match inst {
                Instruction::Mul(a, b) => acc + a * b,
                _ => acc,
            })
            .to_string()
    }

    fn part2(&self) -> String {
        let (result, _) = self
            .instructions
            .iter()
            .fold((0, true), |acc, inst| match (acc, inst) {
                ((val, true), Instruction::Mul(a, b)) => (val + a * b, true),
                ((val, _), Instruction::Do) => (val, true),
                ((val, _), Instruction::Dont) => (val, false),
                _ => acc,
            });
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "161");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "175700056");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example, 2));
        assert_eq!(solution.part2(), "48");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "71668682");
    }
}
