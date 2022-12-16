use std::io::BufRead;

use crate::{day::AoCDay, input};

pub struct Solution {
    depths: Vec<u32>,
}

impl Solution {
    fn count_increasing_depth(&self, group_size: usize) -> usize {
        self.depths
            .windows(group_size + 1)
            .filter(|x| x[0] < x[group_size])
            .count()
    }
}

impl AoCDay for Solution {
    fn title() -> &'static str {
        "Sonar Sweep"
    }

    fn with_input(input: &mut impl BufRead) -> Self {
        let depths = input::read_lines(input)
            .filter(|l| !l.is_empty())
            .map(|line| line.parse::<u32>().unwrap())
            .collect();

        Self { depths }
    }

    fn part1(&self) -> String {
        self.count_increasing_depth(1).to_string()
    }

    fn part2(&self) -> String {
        self.count_increasing_depth(3).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2021 1));
        assert_eq!(day.part1(), "7");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2021 1));
        assert_eq!(day.part1(), "1564");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2021 1));
        assert_eq!(day.part2(), "5");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2021 1));
        assert_eq!(day.part2(), "1611");
    }
}
