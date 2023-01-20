use std::io::{self, BufRead};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::ocr,
};

pub const TITLE: &str = "Cathode-Ray Tube";

const LINE_LENGTH: u8 = 40;
const CENTER_CYCLES: [u8; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i8),
}

pub struct Solution {
    instructions: Vec<Instruction>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, instructions) = instructions(&input).unwrap();

        Self { instructions }
    }
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Instruction::Noop),
            preceded(tag("addx "), complete::i8).map(Instruction::Add),
        )),
    )(input)
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let (_, _, sum_of_signal_strengths) = self.instructions.iter().fold(
            (1, 1i8, 0u16),
            |(cycle, value, mut sum), inst| match inst {
                Instruction::Noop => {
                    if CENTER_CYCLES.contains(&cycle) {
                        sum += value as u16 * cycle as u16;
                    }
                    (cycle + 1, value, sum)
                }
                Instruction::Add(x) => {
                    if CENTER_CYCLES.contains(&cycle) {
                        sum += value as u16 * cycle as u16;
                    } else if CENTER_CYCLES.contains(&(cycle + 1)) {
                        sum += value as u16 * (cycle + 1) as u16;
                    }
                    (cycle + 2, value + x, sum)
                }
            },
        );

        sum_of_signal_strengths.to_string()
    }

    fn part2(&self) -> String {
        let pixels = self.get_pixels();
        let screen = pixels.chunks(LINE_LENGTH as usize).collect::<Vec<_>>();
        ocr::print_image(&screen);
        ocr::read_message(&screen)
    }
}

impl Solution {
    fn get_pixels(&self) -> Vec<bool> {
        self.instructions
            .iter()
            .fold(
                (0, 1i8, vec![]),
                |(cycle, value, mut screen), inst| match inst {
                    Instruction::Noop => {
                        screen.push(is_in_sprite_range(cycle % LINE_LENGTH, value));
                        (cycle + 1, value, screen)
                    }
                    Instruction::Add(x) => {
                        screen.push(is_in_sprite_range(cycle % LINE_LENGTH, value));
                        screen.push(is_in_sprite_range((cycle + 1) % LINE_LENGTH, value));
                        (cycle + 2, value + x, screen)
                    }
                },
            )
            .2
    }
}

fn is_in_sprite_range(cycle: u8, value: i8) -> bool {
    value >= cycle as i8 - 1 && value <= cycle as i8 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{input, output};

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 10));
        assert_eq!(day.part1(), "13140");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 10));
        assert_eq!(day.part1(), "13220");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 10));
        let pixels = day.get_pixels();
        assert_eq!(pixels[..40], [true, true, false, false].repeat(10));
        assert_eq!(
            pixels[40..76],
            [true, true, true, false, false, false].repeat(6)
        );
        assert_eq!(
            pixels[80..120],
            [true, true, true, true, false, false, false, false].repeat(5)
        );
        assert_eq!(
            pixels[120..160],
            [true, true, true, true, true, false, false, false, false, false].repeat(4)
        );
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 10));
        output::disable_debug();
        assert_eq!(day.part2(), "RUAKHBEK");
    }
}
