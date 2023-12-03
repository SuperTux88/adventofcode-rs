use std::io::BufRead;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::parsing::lines_vec,
};

pub const TITLE: &str = "Trebuchet?!";

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub struct Solution {
    lines: Vec<String>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        Self {
            lines: lines_vec(input),
        }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.sum(digits).to_string()
    }

    fn part2(&self) -> String {
        self.sum(numbers).to_string()
    }
}

impl Solution {
    fn sum(&self, parser: fn(&str) -> Box<dyn Iterator<Item = u32> + '_>) -> u32 {
        self.lines.iter().map(|line| value(parser(line))).sum()
    }
}

fn value(mut digits: impl Iterator<Item = u32>) -> u32 {
    let first = digits.next().expect("always needs at least one digit");
    match digits.last() {
        Some(last) => first * 10 + last,
        _ => first * 11,
    }
}

fn digits(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    Box::new(line.chars().filter_map(|c| c.to_digit(10)))
}

fn numbers(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    let iter = (0..line.len()).filter_map(|index| {
        let tail = &line[index..];
        match tail.chars().next().unwrap().to_digit(10) {
            Some(digit) => Some(digit),
            None => NUMBERS
                .iter()
                .enumerate()
                .find_map(|(num, &num_word)| tail.starts_with(num_word).then_some(num as u32)),
        }
    });
    Box::new(iter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_digits() {
        let vals = digits("1a2b3c4d");
        assert_eq!(vals.collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "142");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "56042");
    }

    #[test]
    fn test_numbers() {
        let vals = numbers("twone");
        assert_eq!(vals.collect::<Vec<_>>(), vec![2, 1]);
        let vals = numbers("eightwo");
        assert_eq!(vals.collect::<Vec<_>>(), vec![8, 2]);
        let vals = numbers("a1btwoc3dfour");
        assert_eq!(vals.collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example, 2));
        assert_eq!(solution.part2(), "281");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "55358");
    }
}
