use std::io::BufRead;

use crate::day::AoCDay;

mod day;
pub mod input;

mod y2022 {
    pub mod day1;
}

macro_rules! aoc_solutions {
    ($(($year:ident: $($day:ident),+)),+) => {
        pub fn all_years() -> Vec<u16> {
            vec![$(
                stringify!($year)[1..].parse::<u16>().unwrap(),
            )+]
        }

        pub fn all_days_for_year(year: u16) -> Vec<u8> {
            $(
                if year == stringify!($year)[1..].parse::<u16>().unwrap() {
                    return vec![$(
                        stringify!($day)[3..].parse::<u8>().unwrap(),
                    )+]
                }
            )+
            vec![]
        }

        pub fn run_solution(year: u16, day: u8, input: &mut impl BufRead) {
            match year {
                $(
                    y if y == stringify!($year)[1..].parse::<u16>().unwrap() => {
                        match day {
                            $(
                                d if d == stringify!($day)[3..].parse::<u8>().unwrap() => {
                                    println!("Day {} {}: ", day, y);
                                    day::run_day(<$year::$day::Solution as AoCDay>::with_input(input))
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

aoc_solutions!((y2022: day1));
