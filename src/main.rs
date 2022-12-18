use std::io::{self, BufReader};
use std::path::PathBuf;
use std::process;

use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use adventofcode::{day::Part, input, Solutions};

#[derive(Parser)]
#[command(author, version, about = "Advent of Code soltions in rust.", long_about = None)]
#[command(override_usage = "
\tadventofcode list
\tadventofcode run [-y <year>] [-d <day>] [-p <part>] [-i <input>]
")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available solutions
    List,

    /// Run a solution
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
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

    /// Download and cache input file
    #[arg(long)]
    download: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            println!("Currently implemented solutions:");
            let years = Solutions::years();
            for year in years {
                println!("\t{}: {}", year, num_list(Solutions::days_for_year(year)));
            }
        }

        Commands::Run(args) => {
            let all_days = Solutions::days_for_year(args.year);

            if let Some(day) = args.day {
                if all_days.contains(&day) {
                    match &args.input {
                        Some(stdin) if stdin == "-" => {
                            let mut stdin = BufReader::new(io::stdin());
                            Solutions::run(args.year, day, &args.part, &mut stdin)
                        }
                        Some(path) => match input::read_input(&PathBuf::from(path)) {
                            Ok(mut input) => Solutions::run(args.year, day, &args.part, &mut input),
                            Err(e) => exit_error(e),
                        },
                        None => run_solution_with_default_input(
                            args.year,
                            day,
                            &args.part,
                            args.download,
                        ),
                    };
                } else {
                    exit_error(format!(
                        "No solutions for day {} {} yet, chose one of: {}",
                        day,
                        args.year,
                        num_list(all_days)
                    ));
                }
            } else {
                if args.input.is_some() {
                    exit_error("--input can only be specified when --day is specified".to_string());
                }
                for day in all_days {
                    run_solution_with_default_input(args.year, day, &args.part, args.download);
                }
            }
        }
    }
}

fn run_solution_with_default_input(year: u16, day: u8, part: &Part, download: bool) {
    match input::read_default_input(year, day, download) {
        Ok(mut input) => Solutions::run(year, day, part, &mut input),
        Err(e) => exit_error(e),
    }
}

fn num_list<T: ToString>(list: Vec<T>) -> String {
    list.iter()
        .map(|y| y.to_string())
        .collect::<Vec<String>>()
        .join(", ")
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
        Err(format!(
            "No solutions for {} yet, chose one of: {}",
            year,
            num_list(all_years)
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
