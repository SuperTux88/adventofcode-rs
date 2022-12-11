use crate::day::AoCDay;

use std::io::BufRead;

pub mod day1;

pub fn get_day(day: u8, input: &mut impl BufRead) -> Result<impl AoCDay, &'static str> {
    match day {
        1 => Ok(day1::Solution::with_input(input)),
        _ => Err("Day is not implemented yet"),
    }
}
