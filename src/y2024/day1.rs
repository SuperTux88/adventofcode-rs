use std::io::BufRead;

use itertools::Itertools;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::parsing::lines_iter,
};

pub const TITLE: &str = "Historian Hysteria";

pub struct Solution {
    list1: Vec<u32>,
    list2: Vec<u32>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let (mut list1, mut list2) = (vec![], vec![]);
        lines_iter(input).for_each(|line| {
            let mut parts = line.split("   ");
            list1.push(parts.next().unwrap().parse::<u32>().unwrap());
            list2.push(parts.next().unwrap().parse::<u32>().unwrap());
        });

        list1.sort();
        list2.sort();

        Self { list1, list2 }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let diffs = self
            .list1
            .iter()
            .zip(self.list2.iter())
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
