use std::io::BufRead;

use colored::Colorize;

use crate::day::{AoCDay, Part};

pub mod day;
pub mod input;

pub mod y2021 {
    pub mod day1;
}

pub mod y2022 {
    pub mod day1;
    pub mod day3;
}

macro_rules! aoc_solutions {
    ($(($year:ident: $($day:ident),+)),+) => {
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

        pub fn run(year: u16, day: u8, part: &Part,input: &mut impl BufRead) {
            match year {
                $(
                    y if y == stringify!($year)[1..].parse::<u16>().unwrap() => {
                        match day {
                            $(
                                d if d == stringify!($day)[3..].parse::<u8>().unwrap() => {
                                    println!("Day {} {}: {}",
                                        day, y, <$year::$day::Solution as AoCDay>::title().white().bold());
                                    day::run_day(<$year::$day::Solution as AoCDay>::with_input(input), part)
                                },
                            )+
                            _ => panic!("Day {} {} is not implemented yet", day, y),
                        }
                    }
                )+
                _ => panic!("Year {} is not implemented yet", year),
            }
        }
    };
}

pub struct Solutions {}

impl Solutions {
    #[rustfmt::skip]
    aoc_solutions!(
        (y2021: day1),
        (y2022: day1, day3)
    );
}
