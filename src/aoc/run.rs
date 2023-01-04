use std::time::{Duration, Instant};

use colored::Colorize;

use super::{output, Day, Part};

/// Runs the given part(s) for the given day and returns the results.
pub fn run_day(day: impl Day, part: &Part) -> (Option<String>, Option<String>) {
    match part {
        Part::Part1 => (Some(run_and_print_part(&day, part, true)), None),
        Part::Part2 => (None, Some(run_and_print_part(&day, part, true))),
        Part::Both => (
            Some(run_and_print_part(&day, &Part::Part1, false)),
            Some(run_and_print_part(&day, &Part::Part2, true)),
        ),
    }
}

/// Runs the given part(s) for the given day and returns the duration for benchmark.
pub fn bench_day(day: impl Day, part: &Part) -> (Option<Duration>, Option<Duration>) {
    match part {
        Part::Part1 => (Some(bench_part(&day, part)), None),
        Part::Part2 => (None, Some(bench_part(&day, part))),
        Part::Both => (
            Some(bench_part(&day, &Part::Part1)),
            Some(bench_part(&day, &Part::Part2)),
        ),
    }
}

fn run_part(day: &impl Day, part: &Part) -> String {
    match part {
        Part::Part1 => day.part1(),
        Part::Part2 => day.part2(),
        Part::Both => unreachable!(),
    }
}

fn run_and_print_part(day: &impl Day, part: &Part, last: bool) -> String {
    let result = run_part(day, part);
    output::println(format!(
        "{}─ Part {}: {}",
        if last { '└' } else { '├' },
        part,
        result.green()
    ));
    result
}

fn bench_part(day: &impl Day, part: &Part) -> Duration {
    let start = Instant::now();
    run_part(day, part);
    start.elapsed()
}
