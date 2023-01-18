use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::{
    aoc::{day::DayParser, Day},
    common::grid::{
        directions::{Direction, Turn},
        distance::ManhattenDistance,
        walk::Walk,
    },
};

#[derive(Debug)]
struct Instruction {
    turn: Turn,
    distance: u16,
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
    separated_list1(tag(", "), instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (turn, distance)) = tuple((turn, complete::u16))(input)?;
    Ok((input, Instruction { turn, distance }))
}

fn turn(input: &str) -> IResult<&str, Turn> {
    let (input, turn) = anychar(input)?;
    Ok((
        input,
        match turn {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!("Invalid turn: {}", turn),
        },
    ))
}

impl Day for Solution {
    fn title() -> &'static str {
        "No Time for a Taxicab"
    }

    fn part1(&self) -> String {
        let (target, _) = self.instructions.iter().fold(
            (IVec2::ZERO, Direction::Up),
            |(current, direction), instruction| {
                let new_direction = direction.turn(&instruction.turn);
                (
                    current.move_distance(new_direction, instruction.distance as i32),
                    new_direction,
                )
            },
        );
        target.manhattan_distance(&IVec2::ZERO).to_string()
    }

    fn part2(&self) -> String {
        let mut visited = HashSet::new();
        let mut current = IVec2::ZERO;
        let mut direction = Direction::Up;

        for inst in &self.instructions {
            direction = direction.turn(&inst.turn);
            for _ in 0..inst.distance {
                if !visited.insert(current) {
                    return current.manhattan_distance(&IVec2::ZERO).to_string();
                }
                current = current.move_step(direction);
            }
        }

        panic!("No location visited twice!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(&mut "R2, L3".as_bytes());
        assert_eq!(day.part1(), "5");
        let day = Solution::with_input(&mut "R2, R2, R2".as_bytes());
        assert_eq!(day.part1(), "2");
        let day = Solution::with_input(&mut "R5, L5, R5, R3".as_bytes());
        assert_eq!(day.part1(), "12");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2016 1));
        assert_eq!(day.part1(), "301");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(&mut "R8, R4, R4, R8".as_bytes());
        assert_eq!(day.part2(), "4");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2016 1));
        assert_eq!(day.part2(), "130");
    }
}
