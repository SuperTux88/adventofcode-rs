use std::{
    io::BufRead,
    time::{Duration, Instant},
};

use colored::Colorize;

use crate::Solutions;

use super::{
    day::DaySolution,
    output,
    part::Part,
    results::{BenchResults, Results},
};

pub fn run(year: u16, day: u8, part: &Part, input: &mut dyn BufRead) -> Results {
    let solution = Solutions::get_with_input(year, day, input);
    output::println(format!(
        "Day {} {}: {}",
        day,
        year,
        solution.title().white().bold()
    ));
    run_solution(solution.as_ref(), part)
}

/// Runs the given part(s) for the given day and returns the results.
pub fn run_solution(solution: &dyn DaySolution, part: &Part) -> Results {
    match part {
        Part::Part1 => Results {
            part1: Some(run_and_print_part(solution, part, true)),
            part2: None,
        },
        Part::Part2 => Results {
            part1: None,
            part2: Some(run_and_print_part(solution, part, true)),
        },
        Part::Both => Results {
            part1: Some(run_and_print_part(solution, &Part::Part1, false)),
            part2: Some(run_and_print_part(solution, &Part::Part2, true)),
        },
    }
}

/// Runs the given part(s) for the given day and returns the duration for benchmark.
pub fn bench(year: u16, day: u8, part: &Part, input: &mut dyn BufRead) -> BenchResults {
    let start = std::time::Instant::now();
    let solution = Solutions::get_with_input(year, day, input);
    let parsing = start.elapsed();
    let (part1, part2) = match part {
        Part::Part1 => (Some(bench_part(solution.as_ref(), part)), None),
        Part::Part2 => (None, Some(bench_part(solution.as_ref(), part))),
        Part::Both => (
            Some(bench_part(solution.as_ref(), &Part::Part1)),
            Some(bench_part(solution.as_ref(), &Part::Part2)),
        ),
    };
    let total = start.elapsed();
    BenchResults {
        parsing,
        part1,
        part2,
        total,
    }
}

fn run_part(day: &dyn DaySolution, part: &Part) -> String {
    match part {
        Part::Part1 => day.part1(),
        Part::Part2 => day.part2(),
        Part::Both => unreachable!(),
    }
}

fn run_and_print_part(day: &dyn DaySolution, part: &Part, last: bool) -> String {
    let result = run_part(day, part);
    output::println(format!(
        "{}─ Part {}: {}",
        if last { '└' } else { '├' },
        part,
        result.green()
    ));
    result
}

fn bench_part(day: &dyn DaySolution, part: &Part) -> Duration {
    let start = Instant::now();
    run_part(day, part);
    start.elapsed()
}
