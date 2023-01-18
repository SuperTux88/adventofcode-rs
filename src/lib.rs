use std::io::BufRead;

use colored::Colorize;

use crate::aoc::{
    day::{Day, DayParser},
    output,
    results::{BenchResults, Results},
    run, Part,
};

pub mod aoc;
pub mod common;

macro_rules! aoc_solutions {
    ($(($year:ident: $($day:ident),+)),+) => {
        $(
            pub mod $year {
                $(
                    pub mod $day;
                )+
            }
        )+

        pub struct Solutions {}

        impl Solutions {
            pub fn years() -> Vec<u16> {
                vec![$(
                    stringify!($year)[1..].parse::<u16>().unwrap(),
                )+]
            }

            pub fn days_for_year(year: u16) -> Vec<u8> {
                match format!("y{}", year).as_str() {
                    $(
                        stringify!($year) => vec![$(
                            stringify!($day)[3..].parse::<u8>().unwrap(),
                        )+],
                    )+
                    _ => vec![],
                }
            }

            pub fn run(year: u16, day: u8, part: &Part, input: &mut impl BufRead) -> Results {
                match format!("y{}", year).as_str() {
                    $(
                        stringify!($year) => {
                            match format!("day{}", day).as_str() {
                                $(
                                    stringify!($day) => {
                                        output::println(format!(
                                            "Day {} {}: {}",
                                            day, year, <$year::$day::Solution as Day>::title().white().bold()
                                        ));
                                        let solution = <$year::$day::Solution as DayParser>::with_input(input);
                                        let (part1, part2) = run::run_day(solution, part);
                                        Results { part1, part2 }
                                    },
                                )+
                                _ => panic!("Day {} {} is not implemented yet", day, year),
                            }
                        }
                    )+
                    _ => panic!("Year {} is not implemented yet", year),
                }
            }

            pub fn bench(year: u16, day: u8, part: &Part, input: &mut impl BufRead) -> BenchResults {
                match format!("y{}", year).as_str() {
                    $(
                        stringify!($year) => {
                            match format!("day{}", day).as_str() {
                                $(
                                    stringify!($day) => {
                                        let start = std::time::Instant::now();
                                        let solution = <$year::$day::Solution as DayParser>::with_input(input);
                                        let parsing = start.elapsed();
                                        let (part1, part2) = run::bench_day(solution, part);
                                        let total = start.elapsed();
                                        BenchResults { parsing, part1, part2, total }
                                    },
                                )+
                                _ => panic!("Day {} {} is not implemented yet", day, year),
                            }
                        }
                    )+
                    _ => panic!("Year {} is not implemented yet", year),
                }
            }
        }
    };
}

#[rustfmt::skip]
aoc_solutions!(
    (y2016: day1),
    (y2021: day1),
    (y2022: day1, day3, day4, day6, day10, day13, day16, day17, day23, day24, day25)
);
