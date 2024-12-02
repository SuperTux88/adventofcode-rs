use std::io::{self, BufRead};

use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

use crate::aoc::day::{DayParser, DaySolution};

pub const TITLE: &str = "Red-Nosed Reports";

pub struct Solution {
    reports: Vec<Vec<u8>>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, reports) = reports(input.as_str()).unwrap();
        Self { reports }
    }
}

fn reports(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(newline, report)(input)
}

fn report(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(space1, complete::u8)(input)
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let safe_reports = self.reports.iter().filter(|r| is_safe(r));
        safe_reports.count().to_string()
    }

    fn part2(&self) -> String {
        let safe_reports = self
            .reports
            .iter()
            .filter(|r| is_safe_with_problem_dampener(r));
        safe_reports.count().to_string()
    }
}

fn is_safe(report: &[u8]) -> bool {
    (is_decresing(report) || is_incresing(report)) && diff_between_1_and_3(report)
}

fn is_safe_with_problem_dampener(report: &[u8]) -> bool {
    (0..report.len()).any(|i| {
        let mut report = report.to_owned();
        report.remove(i);
        is_safe(&report)
    })
}

fn is_incresing(report: &[u8]) -> bool {
    report.windows(2).all(|w| w[0] < w[1])
}

fn is_decresing(report: &[u8]) -> bool {
    report.windows(2).all(|w| w[0] > w[1])
}

fn diff_between_1_and_3(report: &[u8]) -> bool {
    report
        .windows(2)
        .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "2");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "407");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "4");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "459");
    }
}
