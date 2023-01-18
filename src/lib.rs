use std::io::BufRead;

use crate::aoc::{
    day::{DayParser, DaySolution},
    output,
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

            pub fn get_with_input(year: u16, day: u8, input: &mut dyn BufRead) -> Box<dyn DaySolution> {
                match format!("y{}", year).as_str() {
                    $(
                        stringify!($year) => {
                            match format!("day{}", day).as_str() {
                                $(
                                    stringify!($day) => Box::new(<$year::$day::Solution as DayParser>::with_input(input)),
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
