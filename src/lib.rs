use std::{io::{BufRead, BufReader}, fs::File};

mod day;

mod y2022;

pub fn run_solution_for(year: u16, day: u8, input: &mut impl BufRead) -> Result<(), &'static str> {
    if day < 1 || day > 25 {
        Err("Day needs to be between 1 and 25")
    } else {
        match year {
            2022 => Ok(day::run_day(y2022::get_day(day, input)?)),
            _ => Err("Year is not implemented yet"),
        }
    }
}

pub fn get_input(year: u16, day: u8) -> Result<impl BufRead, &'static str> {
    let path = format!("input/{}/day{}.txt", year, day);
    Ok(BufReader::new(File::open(path).unwrap()))
}
