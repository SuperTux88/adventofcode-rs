use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

use nom::{
    branch::alt,
    character::complete::{self, char, newline},
    multi::{many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Distress Signal";

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}

pub struct Solution {
    packets: Vec<Packet>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, packets) = packets(&input).unwrap();

        Self { packets }
    }
}

fn packets(input: &str) -> IResult<&str, Vec<Packet>> {
    separated_list1(many1(newline), packet)(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(char('['), separated_list0(char(','), packet), char(']')).map(Packet::List),
        complete::u8.map(Packet::Number),
    ))(input)
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.packets
            .chunks(2)
            .enumerate()
            .filter_map(|(i, pair)| match pair {
                [a, b] => match a.cmp(b) {
                    Ordering::Less => Some(i as u32 + 1),
                    Ordering::Equal => unreachable!("Equal packets"),
                    Ordering::Greater => None,
                },
                _ => panic!("Not a pair"),
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
        let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

        let mut packets = self
            .packets
            .iter()
            .chain([&packet_2, &packet_6])
            .collect::<Vec<_>>();
        packets.sort();

        let pos_2 = packets.iter().position(|p| p == &&packet_2).unwrap() + 1;
        let pos_6 = packets.iter().position(|p| p == &&packet_6).unwrap() + 1;
        (pos_2 * pos_6).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "13");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "5557");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "140");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "22425");
    }
}
