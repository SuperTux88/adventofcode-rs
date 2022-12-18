use std::io::{self, BufReader};
use std::path::PathBuf;
use std::time::Duration;
use std::{env, process};

use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use adventofcode::{day::Part, input, output, Solutions};

const AOC_BENCH_LOOPS: u16 = 10;
const AOC_BENCH_LOOPS_ENV_VAR: &str = "AOC_BENCH_LOOPS";

#[derive(Parser)]
#[command(author, version, about = "Advent of Code soltions in rust.", long_about = None)]
#[command(override_usage = "
\tadventofcode list
\tadventofcode run [-y <year>] [-d <day>] [-p <part>] [-i <input>]
\tadventofcode bench [-y <year>] [-d <day>] [-p <part>] [-i <input>]
")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available solutions
    List,

    /// Run a solution and print results
    Run(RunArgs),

    /// Run a solution and print some benchmark times
    Bench(RunArgs),
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

enum RunMode {
    Results,
    Benchmark,
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

        Commands::Run(args) => run_solutions(&RunMode::Results, args),

        Commands::Bench(args) => {
            if Some("-".to_string()) == args.input {
                exit_error("Benchmarking from stdin is not supported".to_string())
            }

            output::disable_output();
            run_solutions(&RunMode::Benchmark, args)
        }
    }
}

fn run_solutions(mode: &RunMode, args: &RunArgs) {
    let all_days = Solutions::days_for_year(args.year);
    if let Some(day) = args.day {
        if all_days.contains(&day) {
            match &args.input {
                Some(stdin) if stdin == "-" => {
                    let mut stdin = BufReader::new(io::stdin());
                    Solutions::run(args.year, day, &args.part, &mut stdin);
                }
                input => {
                    let input = input_path_or_default(args.year, day, input.clone(), args.download);
                    run_solution_with_mode(args.year, day, &args.part, &input, mode);
                }
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
            let input = input_path_or_default(args.year, day, None, args.download);
            run_solution_with_mode(args.year, day, &args.part, &input, mode);
        }
    }
}

fn input_path_or_default(year: u16, day: u8, input: Option<String>, download: bool) -> PathBuf {
    input.map(PathBuf::from).unwrap_or_else(|| {
        match input::get_default_input_path(year, day, download) {
            Ok(path) => path,
            Err(e) => exit_error(e),
        }
    })
}

fn run_solution_with_mode(year: u16, day: u8, part: &Part, path: &PathBuf, mode: &RunMode) {
    match mode {
        RunMode::Results => {
            run_solution_with_input(year, day, part, path);
        }
        RunMode::Benchmark => {
            print!("{} day {:>2}: ", year, day);
            let (mut parse, mut part1, mut part2, mut total) =
                (Vec::new(), Vec::new(), Vec::new(), Vec::new());

            let loops = env::var(AOC_BENCH_LOOPS_ENV_VAR)
                .map(|v| v.parse::<u16>().unwrap_or(AOC_BENCH_LOOPS))
                .unwrap_or(AOC_BENCH_LOOPS);
            for _i in 0..loops {
                let (p, p1, p2, t) = run_solution_with_input(year, day, part, path);
                parse.push(p);
                part1.push(p1);
                part2.push(p2);
                total.push(t);
            }

            match part {
                Part::Both => println!(
                    "parsing: {} | part 1: {} | part 2: {} | total: {}",
                    print_times(parse),
                    print_times(part1),
                    print_times(part2),
                    print_times(total)
                ),
                part => println!(
                    "parsing: {} | part {}: {} | total: {}",
                    print_times(parse),
                    part,
                    print_times(part1),
                    print_times(total)
                ),
            };
        }
    }
}

fn print_times(times: Vec<Duration>) -> String {
    let avg = times.iter().sum::<Duration>() / times.len() as u32;
    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();
    format!("({:>7.1?} / {:>7.1?} / {:>7.1?})", min, avg, max)
}

fn run_solution_with_input(
    year: u16,
    day: u8,
    part: &Part,
    path: &PathBuf,
) -> (Duration, Duration, Duration, Duration) {
    match input::read_input(path) {
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
    process::exit(1)
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
