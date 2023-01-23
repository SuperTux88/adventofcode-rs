use std::{io::BufRead, path::Path};

use colored::Colorize;

use crate::aoc::{
    day::{Day, DaySolution},
    input, output,
    part::Part,
};

use super::run::Run;

pub struct PrintResults {}

impl Run for PrintResults {
    fn run(day: Day, part: &Part, input_path: &Path) -> Result<(), String> {
        run_and_print_results(day, part, &mut input::read_input(input_path)?);
        Ok(())
    }
}

pub fn run_and_print_results(day: Day, part: &Part, input: &mut dyn BufRead) {
    output::println(format!(
        "Day {} {}: {}",
        day.day,
        day.year,
        day.title.white().bold()
    ));

    let solution = day.parse(input);

    match part {
        Part::Part1 => run_and_print_part(solution.as_ref(), part, true),
        Part::Part2 => run_and_print_part(solution.as_ref(), part, true),
        Part::Both => {
            run_and_print_part(solution.as_ref(), &Part::Part1, false);
            run_and_print_part(solution.as_ref(), &Part::Part2, true);
        }
    }
}

fn run_and_print_part(day: &dyn DaySolution, part: &Part, last: bool) {
    let result = part.run_for(day);
    output::println(format!(
        "{}─ Part {}: {}",
        if last { '└' } else { '├' },
        part,
        result.green()
    ));
}
