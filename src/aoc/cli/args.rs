use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use itertools::join;

use crate::{aoc::part::Part, Solutions};

#[derive(Parser)]
#[command(author, version, about = "Advent of Code soltions in rust.", long_about = None)]
#[command(override_usage = "
\taoc list
\taoc run [-y <year>] [-d <day>] [-p <part>] [-i <input>]
\taoc bench [-y <year>] [-d <day>] [-p <part>] [-i <input>]
")]
pub struct Cli {
    /// When to produce colored output
    #[arg(long, value_enum, value_name = "WHEN", default_value_t = ColorMode::Auto)]
    pub color: ColorMode,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available solutions
    List,

    /// Run a solution and print results
    Run(RunArgs),

    /// Run a solution and print some benchmark times
    Bench(RunArgs),
}

#[derive(Args)]
#[clap(group = ArgGroup::new("input-args").multiple(false))]
pub struct RunArgs {
    /// Year to execute
    #[arg(short, long, default_value_t = 2023, value_parser = parse_year)]
    pub year: u16,

    /// Day to execute
    #[arg(short, long, value_parser = parse_day)]
    pub day: Option<u8>,

    /// Part to execute
    #[arg(short, long, value_enum, default_value_t = Part::Both)]
    pub part: Part,

    /// Input file to use, use '-' for stdin [default: input/<year>/day<day>.txt]
    #[arg(short, long, group = "input-args", requires = "day")]
    pub input: Option<String>,

    /// Download and cache input file
    #[cfg(feature = "online")]
    #[arg(long, group = "input-args")]
    pub download: bool,
}

impl RunArgs {
    pub fn download(&self) -> bool {
        #[cfg(feature = "online")]
        {
            self.download
        }

        #[cfg(not(feature = "online"))]
        {
            false
        }
    }
}

#[derive(ValueEnum, Clone)]
pub enum ColorMode {
    /// Automatically detect if the output is a terminal and use colors if so.
    Auto,
    /// Never produce colored output.
    Never,
    /// Always produce colored output.
    Always,
}

impl ColorMode {
    pub fn set_color_mode_override(&self) {
        match self {
            ColorMode::Always => colored::control::set_override(true),
            ColorMode::Never => colored::control::set_override(false),
            ColorMode::Auto => (),
        };
    }
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
            join(all_years, ", ")
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
