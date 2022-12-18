use std::io::BufRead;

use colored::Colorize;

use crate::day::{AoCDay, Part};

pub mod day;
pub mod input;
pub mod output;

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

            pub fn run(year: u16, day: u8, part: &Part, input: &mut impl BufRead)
                    -> (std::time::Duration, std::time::Duration, std::time::Duration, std::time::Duration) {
                match year {
                    $(
                        y if y == stringify!($year)[1..].parse::<u16>().unwrap() => {
                            match day {
                                $(
                                    d if d == stringify!($day)[3..].parse::<u8>().unwrap() => {
                                        output::print(format!(
                                            "Day {} {}: {}",
                                            day, y, <$year::$day::Solution as AoCDay>::title().white().bold()
                                        ));
                                        let start = std::time::Instant::now();
                                        let solution = <$year::$day::Solution as AoCDay>::with_input(input);
                                        let parsed_time = start.elapsed();
                                        let (part1_time, part2_time) = day::run_day(solution, part);
                                        (parsed_time, part1_time, part2_time, start.elapsed())
                                    },
                                )+
                                _ => panic!("Day {} {} is not implemented yet", day, y),
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
    (y2021: day1),
    (y2022: day1, day3)
);
