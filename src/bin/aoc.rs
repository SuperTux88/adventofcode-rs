use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use colored::Colorize;
use itertools::join;

use adventofcode::{
    aoc::{
        cli::{
            args::{Cli, Commands, RunArgs},
            benchmark::run_benchmark,
        },
        input, output,
        part::Part,
        run,
    },
    Solutions,
};

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
                    let day = Solutions::get(args.year, day);
                    run::run(day, &args.part, &mut stdin);
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

fn run_solution_with_mode(year: u16, day: u8, part: &Part, path: &Path, mode: &RunMode) {
    let day = Solutions::get(year, day);
    match mode {
        RunMode::Results => match input::read_input(path) {
            Ok(mut input) => {
                run::run(day, part, &mut input);
            }
            Err(e) => exit_error(e),
        },
        RunMode::Benchmark => {
            match run_benchmark(day, part, path) {
                Ok(()) => {}
                Err(e) => exit_error(e),
            };
        }
    }
}

fn exit_error(e: String) -> ! {
    eprintln!("{} {}", "error:".red(), e);
    process::exit(1)
}
