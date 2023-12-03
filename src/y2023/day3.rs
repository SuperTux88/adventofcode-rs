use std::{collections::HashMap, io::BufRead};

use glam::IVec2;
use itertools::Itertools;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::{
        grid::{directions::Directions, parse_map},
        parsing::lines_iter,
    },
};

pub const TITLE: &str = "Gear Ratios";

enum Tile {
    Number(u8),
    Part(char),
    Empty,
}

struct PartNumber {
    number: u32,
    part: char,
    part_pos: IVec2,
}

pub struct Solution {
    part_numbers: Vec<PartNumber>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let engine = parse_map(lines_iter(input), |c| match c {
            '.' => Tile::Empty,
            digit if digit.is_ascii_digit() => Tile::Number(digit.to_digit(10).unwrap() as u8),
            symbol => Tile::Part(symbol),
        });

        let size = engine.keys().fold(IVec2::ZERO, |acc, pos| acc.max(*pos));

        let mut current_numbers: Option<Vec<IVec2>> = None;
        let mut part_numbers: Vec<PartNumber> = vec![];
        for y in 0..=size.y {
            for x in 0..=size.x {
                match engine.get(&IVec2::new(x, y)).unwrap() {
                    Tile::Number(_) => {
                        if let Some(mut numbers) = current_numbers {
                            numbers.push(IVec2::new(x, y));
                            current_numbers = Some(numbers);
                        } else {
                            current_numbers = Some(vec![IVec2::new(x, y)]);
                        }
                    }
                    _ => {
                        if let Some(numbers) = current_numbers {
                            if let Some(part_number) = get_part_number(&engine, numbers) {
                                part_numbers.push(part_number);
                            }
                            current_numbers = None;
                        }
                    }
                }
            }
        }

        Self { part_numbers }
    }
}

fn get_part_number(engine: &HashMap<IVec2, Tile>, numbers: Vec<IVec2>) -> Option<PartNumber> {
    let number = numbers
        .iter()
        .map(|pos| match engine.get(pos).unwrap() {
            Tile::Number(number) => *number as u32,
            _ => panic!("Expected number"),
        })
        .fold(0, |acc, digit| acc * 10 + digit);

    numbers
        .iter()
        .flat_map(|pos| pos.neighbors())
        .find_map(|pos| match engine.get(&pos) {
            Some(Tile::Part(part)) => Some(PartNumber {
                number,
                part: *part,
                part_pos: pos,
            }),
            _ => None,
        })
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.part_numbers
            .iter()
            .map(|part_number| part_number.number)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.part_numbers
            .iter()
            .filter(|part_number| part_number.part == '*')
            .into_grouping_map_by(|&part_number| part_number.part_pos)
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|(_pos, part_numbers)| {
                if part_numbers.len() == 2 {
                    Some(part_numbers[0].number * part_numbers[1].number)
                } else {
                    None
                }
            })
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
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "4361");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "553079");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "467835");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "84363105");
    }
}
