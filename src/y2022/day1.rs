use std::io::{self, BufRead};

use crate::day::AoCDay;

pub struct Solution {
    elves_calories: Vec<u32>
}

impl AoCDay for Solution {
    fn with_input(input: &mut impl BufRead) -> Self {
        let input_str = io::read_to_string(input).unwrap();
        let elves = input_str.split("\n\n")
            .map(|elf| elf.lines().map(|c| c.parse::<u32>().unwrap()));
        let mut elves_calories: Vec<u32> = elves.map(|elf| elf.sum()).collect();
        elves_calories.sort();

        Self { elves_calories }
    }

    fn part1(&self) -> String {
        self.elves_calories.last().unwrap().clone().to_string()
    }

    fn part2(&self) -> String {
        self.elves_calories.iter().rev().take(3).sum::<u32>().to_string()
    }
}
