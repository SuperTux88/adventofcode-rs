use std::{io::{BufRead, BufReader}, fs::File};

use crate::day::AoCDay;

mod day;

mod y2022 {
    pub mod day1;
}

macro_rules! aoc_solutions {
    ($(($year:ident: $($day:ident),+)),+) => {
        pub fn run_solution_for(year: u16, day: u8, input: &mut impl BufRead) -> Result<(), String> {
            match year {
                $(
                    y if y == stringify!($year)[1..].parse::<u16>().unwrap() => {
                        match day {
                            $(
                                d if d == stringify!($day)[3..].parse::<u8>().unwrap() => {
                                    println!("Day {} {}: ", day, y);
                                    Ok(day::run_day(<$year::$day::Solution as AoCDay>::with_input(input)))
                                },
                            )+
                            _ => {
                                let mut days: Vec<&str> = vec![];
                                $(
                                    days.push(&stringify!($day)[3..]);
                                )+
                                Err(format!("Day {} {} is not implemented yet, days with solutions in {}: {}", day, y, y, days.join(", ")))
                            },
                        }
                    }
                )+
                _ => {
                    let mut years: Vec<&str> = vec![];
                    $(
                        years.push(&stringify!($year)[1..]);
                    )+
                    Err(format!("Year {} is not implemented yet, years with solutions: {}", year, years.join(", ")))
                },
            }
        }
    };
}

aoc_solutions!((y2022: day1));

pub fn get_input(year: u16, day: u8) -> Result<impl BufRead, &'static str> {
    let path = format!("input/{}/day{}.txt", year, day);
    Ok(BufReader::new(File::open(path).unwrap()))
}
