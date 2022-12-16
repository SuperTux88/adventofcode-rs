use std::io::{self, BufReader};
use std::process;

use clap::Parser;
use colored::Colorize;

use adventofcode::{day::Part, input, Solutions};

#[derive(Parser, Debug)]
#[command(author, version, about = "Advent of Code soltions in rust.", long_about = None)]
#[command(override_usage = "adventofcode [-y <year>] [-d <day>] [-p <part>] [-i <input>]")]
struct Args {
    /// Year to execute
    #[arg(short, long, default_value_t = 2022, value_parser = parse_year)]
    year: u16,

    /// Day to execute
    #[arg(short, long, value_parser = parse_day)]
    day: Option<u8>,

    /// Part to execute
    #[arg(short, long, default_value_t = Part::Both)]
    part: Part,

    /// Input file to use, use '-' for stdin [default: input/<year>/day<day>.txt]
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let all_days = Solutions::days_for_year(args.year);

    if let Some(day) = args.day {
        if all_days.contains(&day) {
            match &args.input {
                Some(stdin) if stdin == "-" => {
                    let mut stdin = BufReader::new(io::stdin());
                    Solutions::run(args.year, day, &args.part, &mut stdin)
                }
                Some(path) => match input::read_input(path) {
                    Ok(mut input) => Solutions::run(args.year, day, &args.part, &mut input),
                    Err(e) => exit_error(e),
                },
                None => run_solution_with_default_input(args.year, day, &args.part),
            };
        } else {
            let all_days: Vec<String> = all_days.iter().map(|d| d.to_string()).collect();
            exit_error(format!(
                "No solutions for day {} {} yet, chose one of: {}",
                day,
                args.year,
                all_days.join(", ")
            ));
        }
    } else {
        if args.input.is_some() {
            exit_error("--input can only be specified when --day is specified".to_string());
        }
        for day in all_days {
            run_solution_with_default_input(args.year, day, &args.part);
        }
    }
}

fn run_solution_with_default_input(year: u16, day: u8, part: &Part) {
    match input::read_default_input(year, day) {
        Ok(mut input) => Solutions::run(year, day, part, &mut input),
        Err(e) => exit_error(e),
    }
}

fn exit_error(e: String) -> ! {
    eprintln!("{} {}", "error:".red(), e);
    process::exit(1);
}

fn parse_year(s: &str) -> Result<u16, String> {
    let year = s.parse().map_err(|_| format!("Invalid year: {}", s))?;
    let all_years = Solutions::years();
    if all_years.contains(&year) {
        Ok(year)
    } else {
        let all_years: Vec<String> = all_years.iter().map(|y| y.to_string()).collect();
        Err(format!(
            "No solutions for {} yet, chose one of: {}",
            year,
            all_years.join(", ")
        ))
    }
}

fn parse_day(s: &str) -> Result<u8, String> {
    let day = s
        .parse::<usize>()
        .map_err(|_| format!("Invalid day: {}", s))?;
    if (1..=25).contains(&day) {
        Ok(day as u8)
    } else {
        Err("Day must be between 1 and 25".to_string())
    }
}
