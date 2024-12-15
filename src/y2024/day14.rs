use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use glam::IVec2;
use itertools::Itertools;
use nom::{
    character::complete::{self, char, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::grid::print_area,
};

pub const TITLE: &str = "Restroom Redoubt";

pub const EXAMPLE_SIZE: IVec2 = IVec2 { x: 11, y: 7 };
pub const INPUT_SIZE: IVec2 = IVec2 { x: 101, y: 103 };

#[derive(Debug, Clone)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

pub struct Solution {
    robots: Vec<Robot>,
    size: IVec2,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, robots) = robots(&input).unwrap();

        let max_x = robots.iter().map(|r| r.position.x).max().unwrap();
        let size = if max_x <= EXAMPLE_SIZE.x {
            EXAMPLE_SIZE
        } else {
            INPUT_SIZE
        };

        Self { robots, size }
    }
}

fn robots(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(newline, robot)(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity)) = separated_pair(ivec2('p'), char(' '), ivec2('v'))(input)?;
    Ok((input, Robot { position, velocity }))
}

fn ivec2(c: char) -> impl Fn(&str) -> IResult<&str, IVec2> {
    move |input: &str| {
        let (input, _) = char(c)(input)?;
        let (input, _) = char('=')(input)?;
        let (input, (x, y)) = separated_pair(complete::i32, char(','), complete::i32)(input)?;
        Ok((input, IVec2 { x, y }))
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let mut robots = self.robots.clone();

        for _ in 1..=100 {
            for robot in &mut robots {
                robot.step(self.size);
            }
        }

        print_map(&robots, self.size);

        robots
            .iter()
            .filter_map(|r| r.get_quadrant(self.size / 2))
            .into_group_map_by(|&q| q)
            .into_values()
            .map(|rs| rs.len())
            .product::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut robots = self.robots.clone();
        let mut second = 0;

        loop {
            second += 1;
            for robot in &mut robots {
                robot.step(self.size);
            }

            if robots_count(&robots).values().all_equal_value() == Ok(&1) {
                break;
            }
        }

        print_map(&robots, self.size);

        second.to_string()
    }
}

impl Robot {
    fn step(&mut self, size: IVec2) {
        self.position = (self.position + self.velocity).rem_euclid(size);
    }

    fn get_quadrant(&self, center: IVec2) -> Option<IVec2> {
        if self.position.x == center.x || self.position.y == center.y {
            None
        } else {
            Some((self.position - center).signum())
        }
    }
}

fn robots_count(robots: &[Robot]) -> HashMap<IVec2, usize> {
    robots
        .iter()
        .into_group_map_by(|r| r.position)
        .into_iter()
        .map(|(p, rs)| (p, rs.len()))
        .collect()
}

fn print_map(robots: &[Robot], size: IVec2) {
    let count = robots_count(robots);

    print_area((IVec2::ZERO, size - 1), |p| {
        count.get(&p).map_or(".".to_owned(), |&c| c.to_string())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "12");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "230172768");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "8087");
    }
}
