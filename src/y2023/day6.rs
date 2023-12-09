use std::io::{self, BufRead};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Wait For It";

pub struct Solution {
    time: Vec<String>,
    distance: Vec<String>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, (time, distance)) = parse(input.as_str()).unwrap();
        Self { time, distance }
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
    separated_pair(
        preceded(tuple((tag("Time:"), space1)), numbers),
        newline,
        preceded(tuple((tag("Distance:"), space1)), numbers),
    )(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(space1, digit1.map(|num: &str| num.to_string()))(input)
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.time
            .iter()
            .zip(self.distance.iter())
            .map(|(time, distance)| calculate_wins(time, distance))
            .product::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        let time = self.time.join("");
        let distance = self.distance.join("");
        calculate_wins(&time, &distance).to_string()
    }
}

fn calculate_wins(time: &str, distance: &str) -> u64 {
    let time = time.parse::<f64>().unwrap();
    let distance = distance.parse::<f64>().unwrap();

    // Use quadratic formula to find the time at which it starts to win against the distance
    let start = (time - (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
    // There is an equal ammount of seconds also at the end, where it doesn't win. So just removing these twice works.
    (time - 2.0 * (start + 1.0).floor() + 1.0) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "288");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "1159152");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "71503");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "41513103");
    }
}
