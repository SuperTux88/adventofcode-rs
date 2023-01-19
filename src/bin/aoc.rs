use std::io::{self, BufReader};
use std::path::PathBuf;
use std::time::Duration;
use std::{env, process};

use clap::Parser;
use colored::Colorize;
use itertools::join;

use adventofcode::{
    aoc::{
        cli::args::{Cli, Commands, RunArgs},
        input, output,
        part::Part,
        run,
    },
    Solutions,
};

const AOC_BENCH_LOOPS: u16 = 10;
const AOC_BENCH_LOOPS_ENV_VAR: &str = "AOC_BENCH_LOOPS";

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
                println!("\t{}: {}", year, join(Solutions::days_for_year(year), ", "));
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
                    run::run(args.year, day, &args.part, &mut stdin);
                }
                input => {
                    let input =
                        input_path_or_default(args.year, day, input.clone(), args.download());
                    run_solution_with_mode(args.year, day, &args.part, &input, mode);
                }
            };
        } else {
            exit_error(format!(
                "No solutions for day {} {} yet, chose one of: {}",
                day,
                args.year,
                join(all_days, ", ")
            ));
        }
    } else {
        if args.input.is_some() {
            exit_error(format!(
                "{} can only be specified when {} is specified",
                "--input".yellow(),
                "--day".yellow()
            ));
        }
        output::disable_debug();
        for day in all_days {
            let input = input_path_or_default(args.year, day, None, args.download());
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
        RunMode::Results => match input::read_input(path) {
            Ok(mut input) => {
                run::run(year, day, part, &mut input);
            }
            Err(e) => exit_error(e),
        },
        RunMode::Benchmark => {
            print!("{} day {:>2}: ", year, day);
            let (mut parse, mut part1, mut part2, mut total) =
                (Vec::new(), Vec::new(), Vec::new(), Vec::new());

            let loops = env::var(AOC_BENCH_LOOPS_ENV_VAR)
                .map(|v| v.parse::<u16>().unwrap_or(AOC_BENCH_LOOPS))
                .unwrap_or(AOC_BENCH_LOOPS);
            for _i in 0..loops {
                let durations = match input::read_input(path) {
                    Ok(mut input) => run::bench(year, day, part, &mut input),
                    Err(e) => exit_error(e),
                };
                parse.push(durations.parsing);
                total.push(durations.total);

                // if only one part was run, use part 1 to collect the durations
                part1.push(durations.part1.unwrap_or_else(|| durations.part2.unwrap()));
                part2.push(durations.part2.unwrap_or_default());
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

fn exit_error(e: String) -> ! {
    eprintln!("{} {}", "error:".red(), e);
    process::exit(1)
}
