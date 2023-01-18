use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use crate::aoc::day::{DayParser, DaySolution};

pub struct Solution {
    datastream: Vec<char>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let datastream = io::read_to_string(input)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        Self { datastream }
    }
}

impl DaySolution for Solution {
    fn title(&self) -> &'static str {
        "Tuning Trouble"
    }

    fn part1(&self) -> String {
        self.find_start(4).to_string()
    }

    fn part2(&self) -> String {
        self.find_start(14).to_string()
    }
}

impl Solution {
    fn find_start(&self, marker_size: usize) -> usize {
        self.datastream
            .windows(marker_size)
            .position(|w| w.iter().collect::<HashSet<&char>>().len() == marker_size)
            .unwrap()
            + marker_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    const EX_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EX_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EX_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EX_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 6));
        assert_eq!(day.part1(), "7");
    }

    #[test]
    fn test_part1_more_examples() {
        assert_eq!(Solution::with_input(&mut EX_1.as_bytes()).part1(), "5");
        assert_eq!(Solution::with_input(&mut EX_2.as_bytes()).part1(), "6");
        assert_eq!(Solution::with_input(&mut EX_3.as_bytes()).part1(), "10");
        assert_eq!(Solution::with_input(&mut EX_4.as_bytes()).part1(), "11");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 6));
        assert_eq!(day.part1(), "1093");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 6));
        assert_eq!(day.part2(), "19");
    }

    #[test]
    fn test_part2_more_examples() {
        assert_eq!(Solution::with_input(&mut EX_1.as_bytes()).part2(), "23");
        assert_eq!(Solution::with_input(&mut EX_2.as_bytes()).part2(), "23");
        assert_eq!(Solution::with_input(&mut EX_3.as_bytes()).part2(), "29");
        assert_eq!(Solution::with_input(&mut EX_4.as_bytes()).part2(), "26");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 6));
        assert_eq!(day.part2(), "3534");
    }
}
