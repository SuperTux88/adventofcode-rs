use std::{io::BufRead, iter};

use itertools::Itertools;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::parsing::lines_iter,
};

pub const TITLE: &str = "Mirage Maintenance";

pub struct Solution {
    chains: Vec<Vec<i32>>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let chains = lines_iter(input)
            .map(|line| {
                line.split(' ')
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        Self { chains }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.chains
            .iter()
            .map(|chain| get_next(chain))
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.chains
            .iter()
            .map(|chain| get_prev(chain))
            .sum::<i32>()
            .to_string()
    }
}

fn get_next(chain: &[i32]) -> i32 {
    get_diffs(chain.to_vec())
        .iter()
        .map(|diffs| *diffs.last().unwrap())
        .sum::<i32>()
}

fn get_prev(chain: &[i32]) -> i32 {
    get_diffs(chain.to_vec())
        .iter()
        .map(|diffs| *diffs.first().unwrap())
        .rev()
        .reduce(|a, b| b - a)
        .unwrap()
}

fn get_diffs(chain: Vec<i32>) -> Vec<Vec<i32>> {
    iter::successors(Some(chain), |chain| {
        chain
            .iter()
            .any(|&i| i != 0)
            .then_some(chain.iter().tuple_windows().map(|(a, b)| b - a).collect())
    })
    .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "114");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "1702218515");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "2");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "925");
    }
}
