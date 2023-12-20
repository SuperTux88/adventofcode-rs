use core::panic;
use std::io::{self, BufRead};

use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{self, anychar, newline, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult, Parser,
};

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::grid::{area::shoelace_area, directions::Direction, walk::Walk},
};

pub const TITLE: &str = "Lavaduct Lagoon";

struct Instruction {
    dir: Direction,
    dist: i32,
    real_dir: Direction,
    real_dist: i32,
}

pub struct Solution {
    instructions: Vec<Instruction>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, instructions) = instructions(input.as_str()).unwrap();
        Self { instructions }
    }
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (dir, dist)) = separated_pair(
        anychar.map(|char| match char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }),
        space1,
        complete::i32,
    )(input)?;
    let (input, (real_dir, real_dist)) =
        delimited(tag(" ("), color_instruction, complete::char(')'))(input)?;

    Ok((
        input,
        Instruction {
            dir,
            dist,
            real_dir,
            real_dist,
        },
    ))
}

fn color_instruction(input: &str) -> IResult<&str, (Direction, i32)> {
    preceded(
        complete::char('#'),
        tuple((take(5usize), complete::u8)).map(|(dist, dir)| {
            let dist = i32::from_str_radix(dist, 16).unwrap();
            let dir = match dir {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => panic!("Invalid direction"),
            };
            (dir, dist)
        }),
    )(input)
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let (perimeter, length) = self.get_perimeter(|i| i.dir, |i| i.dist);
        (shoelace_area(&perimeter) + (length / 2) + 1).to_string()
    }

    fn part2(&self) -> String {
        let (perimeter, length) = self.get_perimeter(|i| i.real_dir, |i| i.real_dist);
        (shoelace_area(&perimeter) + (length / 2) + 1).to_string()
    }
}

impl Solution {
    fn get_perimeter(
        &self,
        dir_fn: fn(&Instruction) -> Direction,
        dist_fn: fn(&Instruction) -> i32,
    ) -> (Vec<IVec2>, i64) {
        let mut current_pos = IVec2::ZERO;
        let mut perimeter_length = 0;

        let perimeter_points = self
            .instructions
            .iter()
            .map(|instruction| {
                let dir = dir_fn(instruction);
                let dist = dist_fn(instruction);
                current_pos = current_pos.move_distance(dir, dist);
                perimeter_length += dist;
                current_pos
            })
            .collect_vec();
        (perimeter_points, perimeter_length as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "62");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "47527");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "952408144115");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "52240187443190");
    }
}
