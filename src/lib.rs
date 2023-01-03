use std::io::BufRead;

use colored::Colorize;

use crate::aoc::{
    output,
    results::{BenchResults, Results},
    run, Day, Part,
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
                $(
                    if year == stringify!($year)[1..].parse::<u16>().unwrap() {
                        return vec![$(
                            stringify!($day)[3..].parse::<u8>().unwrap(),
                        )+]
                    }
                )+
                vec![]
            }

            pub fn run(year: u16, day: u8, part: &Part, input: &mut impl BufRead) -> (Results, BenchResults) {
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
                                        let start = std::time::Instant::now();
                                        let solution = <$year::$day::Solution as Day>::with_input(input);
                                        let parsing_time = start.elapsed();
                                        let (part1, part2) = run::run_day(solution, part);
                                        let total_time = start.elapsed();
                                        (
                                            Results::from((&part1, &part2)),
                                            BenchResults {
                                                parsing: parsing_time,
                                                part1: part1.map(|r| r.duration),
                                                part2: part2.map(|r| r.duration),
                                                total: total_time,
                                            }
                                        )
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
