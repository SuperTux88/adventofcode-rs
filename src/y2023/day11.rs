use std::{collections::HashSet, io::BufRead};

use glam::IVec2;
use itertools::Itertools;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::{
        grid::{distance::ManhattenDistance, minmax::minmax_ivec2, parse_set},
        parsing::lines_iter,
    },
};

pub const TITLE: &str = "Cosmic Expansion";

pub struct Solution {
    galaxies: HashSet<IVec2>,
    empty_columns: Vec<i32>,
    empty_rows: Vec<i32>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let galaxies = parse_set(lines_iter(input));
        let (_, max) = minmax_ivec2(galaxies.iter());
        let empty_columns = (0..=max.x)
            .filter(|&x| !galaxies.iter().any(|g| g.x == x))
            .collect_vec();
        let empty_rows = (0..=max.y)
            .filter(|&y| !galaxies.iter().any(|g| g.y == y))
            .collect_vec();

        Self {
            galaxies,
            empty_columns,
            empty_rows,
        }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.expanded_distances(1).to_string()
    }

    fn part2(&self) -> String {
        self.expanded_distances(999999).to_string()
    }
}

impl Solution {
    fn expanded_distances(&self, expand: u64) -> u64 {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| {
                a.manhattan_distance(b) as u64
                    + count_empty_in_range(&self.empty_columns, &a.x, &b.x) * expand
                    + count_empty_in_range(&self.empty_rows, &a.y, &b.y) * expand
            })
            .sum::<u64>()
    }
}

fn count_empty_in_range(empty: &[i32], a: &i32, b: &i32) -> u64 {
    let range = if a < b { a..=b } else { b..=a };
    empty.iter().filter(|e| range.contains(e)).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "374");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "9556896");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.expanded_distances(9), 1030);
        assert_eq!(solution.expanded_distances(99), 8410);
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "685038186836");
    }
}
