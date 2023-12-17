use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

use itertools::Itertools;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::grid::parse_vec,
};

pub const TITLE: &str = "Point of Incidence";

pub struct Solution {
    patterns: Vec<Vec<Vec<bool>>>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let patterns = input
            .split("\n\n")
            .map(|pattern: &str| parse_vec(pattern.lines().map(|l| l.to_string()), |c| c == '#'))
            .collect_vec();

        Self { patterns }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.patterns
            .iter()
            .map(|pattern| find_mirror_imperfections(pattern, 0))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.patterns
            .iter()
            .map(|pattern| find_mirror_imperfections(pattern, 1))
            .sum::<usize>()
            .to_string()
    }
}

fn find_mirror_imperfections(pattern: &[Vec<bool>], imperfections: usize) -> usize {
    let (last_x, last_y) = (pattern.first().unwrap().len() - 1, pattern.len() - 1);

    let mut x_imperfections = (0..last_x).map(|mirror_at| {
        mirror_range(mirror_at, last_x)
            .map(|x| {
                let mirror_x = mirror_pos(mirror_at, x);
                (0..=last_y)
                    .filter(|&y| pattern[y][x] != pattern[y][mirror_x])
                    .count()
            })
            .sum::<usize>()
    });

    let mut y_imperfection = (0..last_y).map(|mirror_at| {
        mirror_range(mirror_at, last_y)
            .map(|y| {
                let mirror_y = mirror_pos(mirror_at, y);
                (0..=last_x)
                    .filter(|&x| pattern[y][x] != pattern[mirror_y][x])
                    .count()
            })
            .sum::<usize>()
    });

    x_imperfections
        .find_position(|&count| count == imperfections)
        .map(|(x, _)| x + 1)
        .unwrap_or_else(|| {
            y_imperfection
                .find_position(|&count| count == imperfections)
                .map(|(y, _)| (y + 1) * 100)
                .unwrap_or(0)
        })
}

fn mirror_range(mirror_at: usize, last_index: usize) -> RangeInclusive<usize> {
    if mirror_at < last_index / 2 {
        0..=mirror_at
    } else {
        mirror_at + 1..=last_index
    }
}

fn mirror_pos(mirror_at: usize, index: usize) -> usize {
    mirror_at * 2 + 1 - index
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_mirror_range() {
        assert_eq!(mirror_range(0, 4), 0..=0);
        assert_eq!(mirror_range(1, 4), 0..=1);
        assert_eq!(mirror_range(2, 4), 3..=4);
        assert_eq!(mirror_range(3, 4), 4..=4);

        assert_eq!(mirror_range(0, 5), 0..=0);
        assert_eq!(mirror_range(1, 5), 0..=1);
        assert_eq!(mirror_range(2, 5), 3..=5);
        assert_eq!(mirror_range(3, 5), 4..=5);
        assert_eq!(mirror_range(4, 5), 5..=5);
    }

    #[test]
    fn test_mirror_pos() {
        assert_eq!(mirror_pos(0, 0), 1);
        assert_eq!(mirror_pos(0, 1), 0);

        assert_eq!(mirror_pos(1, 0), 3);
        assert_eq!(mirror_pos(1, 1), 2);
        assert_eq!(mirror_pos(1, 2), 1);
        assert_eq!(mirror_pos(1, 3), 0);
    }

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "405");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "30518");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "400");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "36735");
    }
}
