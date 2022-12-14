use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_default_input_path(year: u16, day: u8) -> String {
    format!("input/{}/day{}.txt", year, day)
}

pub fn read_input(path: String) -> Result<impl BufRead, String> {
    let file = File::open(&path).map_err(|_| format!("Input file not found: {}", path))?;
    Ok(BufReader::new(file))
}
