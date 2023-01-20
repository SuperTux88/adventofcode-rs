use std::io::BufRead;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::parsing::lines_iter,
};

pub const TITLE: &str = "Full of Hot Air";

pub struct Solution {
    numbers: Vec<u64>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let numbers = lines_iter(input).map(|l| parse_snafu(&l)).collect();
        Self { numbers }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let sum = self.numbers.iter().sum::<u64>();
        to_snafu(sum)
    }

    fn part2(&self) -> String {
        "Merry Christmas!".to_string()
    }
}

fn parse_snafu(input: &str) -> u64 {
    input.chars().enumerate().fold(0i64, |acc, (i, c)| {
        let value = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid character: {}", c),
        };
        acc + value * 5i64.pow((input.len() - i - 1) as u32)
    }) as u64
}

fn to_snafu(value: u64) -> String {
    let mut value = value;
    let mut result = String::new();

    while value > 0 {
        let digit = value % 5;
        value = (value + 2) / 5;

        result.push(match digit {
            2 => '2',
            1 => '1',
            0 => '0',
            4 => '-', // -1
            3 => '=', // -2
            _ => panic!("Invalid digit: {}", digit),
        });
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_parse_snafu() {
        assert_eq!(parse_snafu("1"), 1);
        assert_eq!(parse_snafu("2"), 2);
        assert_eq!(parse_snafu("1="), 3);
        assert_eq!(parse_snafu("1-"), 4);
        assert_eq!(parse_snafu("10"), 5);
        assert_eq!(parse_snafu("11"), 6);
        assert_eq!(parse_snafu("12"), 7);
        assert_eq!(parse_snafu("2="), 8);
        assert_eq!(parse_snafu("2-"), 9);
        assert_eq!(parse_snafu("20"), 10);
        assert_eq!(parse_snafu("1=0"), 15);
        assert_eq!(parse_snafu("1-0"), 20);
        assert_eq!(parse_snafu("1=11-2"), 2022);
        assert_eq!(parse_snafu("1-0---0"), 12345);
        assert_eq!(parse_snafu("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }
    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 25));
        assert_eq!(day.part1(), "2=-1=0");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 25));
        assert_eq!(day.part1(), "2-2=12=1-=-1=000=222");
    }
}
