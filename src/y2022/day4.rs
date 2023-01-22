use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Camp Cleanup";

struct Pair {
    range_a: RangeInclusive<u32>,
    range_b: RangeInclusive<u32>,
}

impl Pair {
    fn includes(&self) -> bool {
        let a_contains_b = self.range_a.contains(self.range_b.start())
            && self.range_a.contains(self.range_b.end());
        let b_contains_a = self.range_b.contains(self.range_a.start())
            && self.range_b.contains(self.range_a.end());
        a_contains_b || b_contains_a
    }

    fn overlaps(&self) -> bool {
        self.range_a.contains(self.range_b.start()) || self.range_b.contains(self.range_a.start())
    }
}

pub struct Solution {
    pairs: Vec<Pair>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, pairs) = pairs(input.as_str()).unwrap();
        Self { pairs }
    }
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(newline, pair)(input)
}

fn pair(input: &str) -> IResult<&str, Pair> {
    let (input, (range_a, range_b)) = separated_pair(range, tag(","), range)(input)?;
    Ok((input, Pair { range_a, range_b }))
}

fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
    Ok((input, start..=end))
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.pairs
            .iter()
            .filter(|p| p.includes())
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.pairs
            .iter()
            .filter(|p| p.overlaps())
            .count()
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
        assert_eq!(solution.part1(), "2");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "515");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "4");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "883");
    }
}
