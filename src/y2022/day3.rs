use std::io::BufRead;

use crate::{common::parsing::lines_vec, day::AoCDay};

pub struct Solution {
    lines: Vec<String>,
}

impl AoCDay for Solution {
    fn title() -> &'static str {
        "Rucksack Reorganization"
    }

    fn with_input(input: &mut impl BufRead) -> Self {
        Self {
            lines: lines_vec(input),
        }
    }

    fn part1(&self) -> String {
        self.lines
            .iter()
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                a.chars().find(|&i| b.contains(i)).unwrap()
            })
            .map(|item| priority(item) as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.lines
            .chunks(3)
            .map(|chunk| {
                chunk[0]
                    .chars()
                    .find(|&i| chunk[1].contains(i) && chunk[2].contains(i))
                    .unwrap()
            })
            .map(|item| priority(item) as u32)
            .sum::<u32>()
            .to_string()
    }
}

fn priority(item: char) -> u8 {
    if item.is_lowercase() {
        (item as u8) - 97 + 1
    } else {
        (item as u8) - 65 + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 3));
        assert_eq!(day.part1(), "157");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 3));
        assert_eq!(day.part1(), "7850");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 3));
        assert_eq!(day.part2(), "70");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 3));
        assert_eq!(day.part2(), "2581");
    }
}
