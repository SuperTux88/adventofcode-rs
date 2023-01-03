use std::time::Instant;

use colored::Colorize;

use super::results::PartResult;

use super::{output, Day, Part};

pub fn run_day(day: impl Day, part: &Part) -> (Option<PartResult>, Option<PartResult>) {
    match part {
        Part::Part1 => (Some(run_part(&day, part, true)), None),
        Part::Part2 => (None, Some(run_part(&day, part, true))),
        Part::Both => (
            Some(run_part(&day, &Part::Part1, false)),
            Some(run_part(&day, &Part::Part2, true)),
        ),
    }
}

fn run_part(day: &impl Day, part: &Part, last: bool) -> PartResult {
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
