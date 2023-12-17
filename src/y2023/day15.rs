use std::{
    array::from_fn,
    io::{self, BufRead},
};

use itertools::Itertools;

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Lens Library";

pub struct Solution {
    sequence: Vec<String>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let sequence = input.trim().split(',').map(|s| s.to_string()).collect_vec();
        Self { sequence }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.sequence
            .iter()
            .map(|s| hash(s) as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut boxes: [Vec<(&str, u8)>; 256] = from_fn(|_| Vec::new());

        for s in &self.sequence {
            match s.split_once('=') {
                Some((key, value)) => {
                    let hash = hash(key) as usize;
                    if let Some(index) = boxes[hash].iter().position(|(k, _)| k == &key) {
                        boxes[hash][index].1 = value.parse().unwrap();
                    } else {
                        boxes[hash].push((key, value.parse().unwrap()));
                    }
                }
                None => {
                    let key = &s[0..s.len() - 1];
                    let hash = hash(key);
                    if let Some(index) = boxes[hash as usize].iter().position(|(k, _)| k == &key) {
                        boxes[hash as usize].remove(index);
                    }
                }
            }
        }

        boxes
            .into_iter()
            .enumerate()
            .flat_map(|(box_index, r#box)| {
                r#box
                    .into_iter()
                    .enumerate()
                    .map(move |(slot, (_, v))| (box_index + 1) * (slot + 1) * v as usize)
            })
            .sum::<usize>()
            .to_string()
    }
}

fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0, |hash, c| hash.wrapping_add(c as u8).wrapping_mul(17))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "1320");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "516657");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "145");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "210906");
    }
}
