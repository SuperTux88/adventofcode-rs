use std::{fmt::Display, io::BufRead, str::FromStr};

pub trait AoCDay {
    fn title() -> &'static str;

    fn with_input(input: &mut impl BufRead) -> Self;

    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Part {
    Part1,
    Part2,
    Both,
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
