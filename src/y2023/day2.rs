use std::io::{self, BufRead};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Cube Conundrum";

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cubes {
    color: Color,
    count: u8,
}

#[derive(Debug)]
struct Game {
    id: u8,
    // Sets of cubes don't really matter, so just adding all the cubes to one big list is fine.
    cubes: Vec<Cubes>,
}

pub struct Solution {
    games: Vec<Game>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, games) = games(input.as_str()).unwrap();
        Self { games }
    }
}

fn games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, game)(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, (id, cubes)) = tuple((
        delimited(tag("Game "), complete::u8, tag(": ")),
        separated_list1(alt((tag("; "), tag(", "))), cubes),
    ))(input)?;
    Ok((input, Game { id, cubes }))
}

fn cubes(input: &str) -> IResult<&str, Cubes> {
    alt((
        terminated(complete::u8, tag(" red")).map(|count| Cubes {
            color: Color::Red,
            count,
        }),
        terminated(complete::u8, tag(" green")).map(|count| Cubes {
            color: Color::Green,
            count,
        }),
        terminated(complete::u8, tag(" blue")).map(|count| Cubes {
            color: Color::Blue,
            count,
        }),
    ))(input)
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.games
            .iter()
            .filter(|game| game.is_valid())
            .map(|game| game.id as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.games
            .iter()
            .map(|game| game.power_of_minimal_cubes())
            .sum::<u32>()
            .to_string()
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.cubes.iter().all(|cube| match cube.color {
            Color::Red => cube.count <= 12,
            Color::Green => cube.count <= 13,
            Color::Blue => cube.count <= 14,
        })
    }

    fn power_of_minimal_cubes(&self) -> u32 {
        let max_by_color = &self
            .cubes
            .iter()
            .into_grouping_map_by(|&cubes| &cubes.color)
            .max_by_key(|_color, &cubes| cubes.count);
        let minimal_number_of_cubes = max_by_color
            .iter()
            .map(|(_color, &cubes)| cubes.count as u32);
        minimal_number_of_cubes.product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "8");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "2156");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "2286");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "66909");
    }
}
