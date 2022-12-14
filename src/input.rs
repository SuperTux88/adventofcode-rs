use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Returns the path to the input file for the given year and day.
///
/// ```
/// # use adventofcode::input;
/// let path = input::get_default_input_path(2022, 1);
/// # assert_eq!(path, "input/2022/day1.txt");
/// ```
pub fn get_default_input_path(year: u16, day: u8) -> String {
    format!("input/{}/day{}.txt", year, day)
}

pub fn read_input(path: &str) -> Result<impl BufRead, String> {
    let file = File::open(path).map_err(|_| format!("Input file not found: {}", path))?;
    Ok(BufReader::new(file))
}

pub fn read_default_input(year: u16, day: u8) -> Result<impl BufRead, String> {
    read_input(&get_default_input_path(year, day))
}

#[macro_export]
macro_rules! input {
    (input: $year:tt $day:tt) => {
        &mut input::read_input(&input::get_default_input_path($year, $day)).unwrap()
    };
    (example: $year:tt $day:tt) => {
        &mut input::read_input(&format!("input/{}/example/day{}.txt", $year, $day)).unwrap()
    };
}
