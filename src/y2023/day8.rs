use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::{fold_many1, many1},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};
use num::integer::lcm;

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Haunted Wasteland";

struct Branch {
    left: String,
    right: String,
}

enum Direction {
    Left,
    Right,
}

pub struct Solution {
    instructions: Vec<Direction>,
    network: HashMap<String, Branch>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, (instructions, network)) = parser(input.as_str()).unwrap();
        Self {
            instructions,
            network,
        }
    }
}

fn parser(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<String, Branch>)> {
    separated_pair(instructions, multispace1, network)(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
    )))(input)
}

fn network(input: &str) -> IResult<&str, HashMap<String, Branch>> {
    fold_many1(
        terminated(
            separated_pair(complete::alphanumeric1, tag(" = "), branch),
            newline,
        ),
        HashMap::new,
        |mut map, (key, branch)| {
            map.insert(key.to_string(), branch);
            map
        },
    )(input)
}

fn branch(input: &str) -> IResult<&str, Branch> {
    let (input, (left, right)) = separated_pair(
        preceded(tag("("), complete::alphanumeric1),
        tag(", "),
        terminated(complete::alphanumeric1, tag(")")),
    )(input)?;
    Ok((
        input,
        Branch {
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.find_target("AAA", |node| node == "ZZZ").to_string()
    }

    fn part2(&self) -> String {
        self.network
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|node| self.find_target(node, |node| node.ends_with('Z')))
            .reduce(lcm)
            .unwrap()
            .to_string()
    }
}

impl Solution {
    fn find_target(&self, start: &str, is_target: fn(&str) -> bool) -> u64 {
        let mut current_node = start;
        self.instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(dist, instr)| {
                let next_node = match instr {
                    Direction::Left => &self.network[current_node].left,
                    Direction::Right => &self.network[current_node].right,
                };
                if is_target(next_node) {
                    Some(dist as u64 + 1)
                } else {
                    current_node = next_node;
                    None
                }
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example, "1_1"));
        assert_eq!(solution.part1(), "2");
    }

    #[test]
    fn test_part1_example_2() {
        let solution = Solution::with_input(input!(example, "1_2"));
        assert_eq!(solution.part1(), "6");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "13019");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example, 2));
        assert_eq!(solution.part2(), "6");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "13524038372771");
    }
}
