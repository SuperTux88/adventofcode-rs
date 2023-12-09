use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Scratchcards";

#[derive(Debug)]
struct Card {
    _id: u16,
    winning_numbers: BTreeSet<u8>,
    numbers: BTreeSet<u8>,
}

pub struct Solution {
    cards: Vec<Card>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, cards) = cards(input.as_str()).unwrap();
        Self { cards }
    }
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, card)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(
        tuple((tag("Card"), space1)),
        complete::u16,
        tuple((tag(":"), space1)),
    )(input)?;
    let (input, (winning_numbers, numbers)) =
        separated_pair(numbers, tuple((space1, tag("|"), space1)), numbers)(input)?;
    Ok((
        input,
        Card {
            _id: id,
            winning_numbers: winning_numbers.into_iter().collect(),
            numbers: numbers.into_iter().collect(),
        },
    ))
}

fn numbers(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(space1, complete::u8)(input)
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.cards
            .iter()
            .map(|card| card.score())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut card_counts = vec![1; self.cards.len()];

        self.cards.iter().enumerate().for_each(|(i, card)| {
            (1..=card.winning_count()).for_each(|j| {
                card_counts[i + j] += card_counts[i];
            })
        });

        card_counts.iter().sum::<u32>().to_string()
    }
}

impl Card {
    fn winning_count(&self) -> usize {
        self.numbers.intersection(&self.winning_numbers).count()
    }

    fn score(&self) -> u32 {
        match self.winning_count().checked_sub(1) {
            None => 0,
            Some(c) => 2u32.pow(c as u32),
        }
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
        assert_eq!(solution.part1(), "21919");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "30");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "9881048");
    }
}
