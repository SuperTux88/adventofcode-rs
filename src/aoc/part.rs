use std::{fmt::Display, str::FromStr};

use clap::ValueEnum;

use super::day::DaySolution;

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    #[value(name = "1")]
    Part1,
    #[value(name = "2")]
    Part2,
    Both,
}

impl Part {
    pub fn values() -> Vec<Self> {
        vec![Self::Part1, Self::Part2, Self::Both]
    }

    pub fn run_for(&self, day: &dyn DaySolution) -> String {
        match self {
            Part::Part1 => day.part1(),
            Part::Part2 => day.part2(),
            Part::Both => format!("{} / {}", day.part1(), day.part2()),
        }
    }
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "part1" | "1" => Ok(Self::Part1),
            "part2" | "2" => Ok(Self::Part2),
            "both" | "b" => Ok(Self::Both),
            _ => Err("Unknown part: use 1, 2 or both"),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => write!(f, "1"),
            Part::Part2 => write!(f, "2"),
            Part::Both => write!(f, "both"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_for() {
        struct TestSolution {}

        impl DaySolution for TestSolution {
            fn part1(&self) -> String {
                "result1".to_string()
            }

            fn part2(&self) -> String {
                "result2".to_string()
            }
        }

        let solution = TestSolution {};

        assert_eq!(Part::Part1.run_for(&solution), "result1");
        assert_eq!(Part::Part2.run_for(&solution), "result2");
        assert_eq!(Part::Both.run_for(&solution), "result1 / result2");
    }
}
