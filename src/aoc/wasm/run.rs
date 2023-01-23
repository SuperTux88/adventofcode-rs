use std::io::BufRead;

use crate::aoc::{day::Day, part::Part};

pub struct Results {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

/// Runs the given part(s) for the given day and returns the results.
pub fn run(day: Day, part: &Part, input: &mut dyn BufRead) -> Results {
    let solution = day.parse(input);
    match part {
        Part::Part1 => Results {
            part1: Some(part.run_for(solution.as_ref())),
            part2: None,
        },
        Part::Part2 => Results {
            part1: None,
            part2: Some(part.run_for(solution.as_ref())),
        },
        Part::Both => Results {
            part1: Some(Part::Part1.run_for(solution.as_ref())),
            part2: Some(Part::Part2.run_for(solution.as_ref())),
        },
    }
}
