use std::{collections::HashMap, io::BufRead};

use itertools::{Itertools, Position};

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::parsing::lines_iter,
};

pub const TITLE: &str = "Camel Cards";

const CARD_SCORES: &str = "X23456789TJQKA";

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    card_values: Vec<usize>,
    card_counts: HashMap<char, usize>,
    bid: u16,
}

pub struct Solution {
    hands: Vec<Hand>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let hands = lines_iter(input)
            .map(|line| {
                let (cards, bid) = line.split_once(' ').unwrap();
                parse_cards(cards, bid)
            })
            .collect();
        Self { hands }
    }
}

fn parse_cards(cards: &str, bid: &str) -> Hand {
    let card_values = cards
        .chars()
        .map(|card| CARD_SCORES.find(card).unwrap())
        .collect();

    let card_counts = cards.chars().counts();

    Hand {
        card_values,
        card_counts,
        bid: bid.parse().unwrap(),
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        calculate_total_score(&self.hands).to_string()
    }

    fn part2(&self) -> String {
        let original_joker_value = CARD_SCORES.find('J').unwrap();
        let hands_with_jocker = self
            .hands
            .iter()
            .map(|hand| match hand.card_counts.get(&'J') {
                None => hand.clone(),
                Some(5) => Hand {
                    card_values: vec![0; 5],
                    ..hand.clone()
                },
                Some(joker_count) => {
                    let joker_card_values = hand
                        .card_values
                        .iter()
                        .map(|&value| {
                            if value == original_joker_value {
                                0
                            } else {
                                value
                            }
                        })
                        .collect();

                    let card_counts = hand
                        .card_counts
                        .iter()
                        .filter(|(&card, _count)| card != 'J')
                        .sorted_by_key(|(_card, &count)| count)
                        .with_position()
                        .map(|(position, (&card, &count))| match position {
                            Position::Last | Position::Only => (card, count + joker_count),
                            _ => (card, count),
                        })
                        .collect();

                    Hand {
                        card_values: joker_card_values,
                        card_counts,
                        bid: hand.bid,
                    }
                }
            })
            .collect_vec();

        calculate_total_score(&hands_with_jocker).to_string()
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        use HandType::*;

        match self.card_counts.values().sorted().as_slice() {
            [1, 1, 1, 1, 1] => HighCard,
            [1, 1, 1, 2] => OnePair,
            [1, 2, 2] => TwoPair,
            [1, 1, 3] => ThreeOfAKind,
            [2, 3] => FullHouse,
            [1, 4] => FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

fn calculate_total_score(hands: &[Hand]) -> u32 {
    hands
        .iter()
        .sorted_by_key(|&hand| (hand.hand_type(), &hand.card_values))
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "6440");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "253603890");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "5905");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "253630098");
    }
}
