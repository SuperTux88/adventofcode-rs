use std::io::BufRead;

use colored::Colorize;

use super::{
    day::{Day, DaySolution},
    output,
    part::Part,
    results::Results,
};

/// Runs the given part(s) for the given day and returns the results.
pub fn run(day: Day, part: &Part, input: &mut dyn BufRead) -> Results {
    output::println(format!(
        "Day {} {}: {}",
        day.day,
        day.year,
        day.title.white().bold()
    ));

    let solution = day.parse(input);
    match part {
        Part::Part1 => Results {
            part1: Some(run_and_print_part(solution.as_ref(), part, true)),
            part2: None,
        },
        Part::Part2 => Results {
            part1: None,
            part2: Some(run_and_print_part(solution.as_ref(), part, true)),
        },
        Part::Both => Results {
            part1: Some(run_and_print_part(solution.as_ref(), &Part::Part1, false)),
            part2: Some(run_and_print_part(solution.as_ref(), &Part::Part2, true)),
        },
    }
}

pub fn run_part(day: &dyn DaySolution, part: &Part) -> String {
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
