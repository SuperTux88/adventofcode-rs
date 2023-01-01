use std::time::{Duration, Instant};

use colored::Colorize;

use crate::{
    day::{AoCDay, Part},
    output,
};

pub struct PartResult {
    pub result: String,
    pub duration: Duration,
}

pub struct Results {
    #[allow(dead_code)]
    part1: Option<String>,
    #[allow(dead_code)]
    part2: Option<String>,
}

impl Results {
    pub fn from(results: (&Option<PartResult>, &Option<PartResult>)) -> Self {
        Self {
            part1: results.0.as_ref().map(|r| r.result.clone()),
            part2: results.1.as_ref().map(|r| r.result.clone()),
        }
    }
}

pub struct BenchResults {
    pub parsing: Duration,
    pub part1: Option<Duration>,
    pub part2: Option<Duration>,
    pub total: Duration,
}

pub fn run_day(day: impl AoCDay, part: &Part) -> (Option<PartResult>, Option<PartResult>) {
    match part {
        Part::Part1 => (Some(run_part(&day, part, true)), None),
        Part::Part2 => (None, Some(run_part(&day, part, true))),
        Part::Both => (
            Some(run_part(&day, &Part::Part1, false)),
            Some(run_part(&day, &Part::Part2, true)),
        ),
    }
}

fn run_part(day: &impl AoCDay, part: &Part, last: bool) -> PartResult {
    let start = Instant::now();
    let result = match part {
        Part::Part1 => day.part1(),
        Part::Part2 => day.part2(),
        Part::Both => unreachable!(),
    };
    output::println(format!(
        "{}─ Part {}: {}",
        if last { '└' } else { '├' },
        part,
        result.green()
    ));
    PartResult {
        result,
        duration: start.elapsed(),
    }
}
