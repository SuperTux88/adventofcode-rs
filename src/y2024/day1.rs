use std::io::{self, BufRead};

use itertools::Itertools;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Historian Hysteria";

pub struct Solution {
    list1: Vec<u32>,
    list2: Vec<u32>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, (list1, list2)) = lists(input.as_str()).unwrap();
        Self { list1, list2 }
    }
}

fn lists(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, pairs) = separated_list1(newline, pair)(input)?;
    let (list1, list2) = pairs.into_iter().unzip();
    Ok((input, (list1, list2)))
}

fn pair(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (a, b)) = separated_pair(complete::u32, space1, complete::u32)(input)?;
    Ok((input, (a, b)))
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let diffs = self
            .list1
            .iter()
            .sorted_unstable()
            .zip(self.list2.iter().sorted_unstable())
            .map(|(&a, &b)| a.abs_diff(b));
        diffs.sum::<u32>().to_string()
    }

    fn part2(&self) -> String {
        let list2_counts = self.list2.iter().counts();
        let scores = self.list1.iter().map(|a| {
            list2_counts
                .get(a)
                .map(|&b| b * (*a as usize))
                .unwrap_or_default()
        });
        scores.sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "11");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "1579939");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "31");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "20351745");
    }
}
