use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[cfg(feature = "online")]
use super::online;

/// Returns the path to the input file for the given year and day.
///
/// ```
/// # use adventofcode::aoc::input;
/// let path = input::get_input_subpath(2022, 1);
/// # assert_eq!(path.as_path().display().to_string(), "input/2022/day1.txt");
/// ```
pub fn get_input_subpath(year: u16, day: u8) -> PathBuf {
    PathBuf::from(format!("input/{}/day{}.txt", year, day))
}

/// Reads the input file
pub fn read_input(path: &PathBuf) -> Result<impl BufRead, String> {
    let file = File::open(path).map_err(|error| {
        format!(
            "Error opening input file ({}): {}",
            path.as_path().display(),
            error
        )
    })?;
    Ok(BufReader::new(file))
}

/// Returns the path to the input file for the given year and day.
/// If `download` is true or if the default file doesn't exist,
/// the input is downloaded and the path to the download is returned.
pub fn get_default_input_path(year: u16, day: u8, download: bool) -> Result<PathBuf, String> {
    if download {
        #[cfg(feature = "online")]
        {
            online::get_input_cache_path_and_download_if_needed(year, day)
        }

        #[cfg(not(feature = "online"))]
        {
            Err("Online features are disabled in this build".to_string())
        }
    } else {
        match get_input_subpath(year, day) {
            path if path.exists() => Ok(path),

            #[cfg(feature = "online")]
            path => {
                online::get_input_cache_path_and_download_if_needed(year, day).map_err(|error| {
                    format!(
                        "Input file ({}) doesn't exist and error downloading input: {}",
                        path.as_path().display(),
                        error
                    )
                })
            }

            #[cfg(not(feature = "online"))]
            path => Err(format!(
                "Input file ({}) doesn't exist",
                path.as_path().display()
            )),
        }
    }
}

#[macro_export]
macro_rules! input {
    (input) => {{
        let (year, day) = $crate::aoc::day::parse_year_and_day_from_module(module_path!());
        &mut $crate::aoc::input::read_input(&$crate::aoc::input::get_input_subpath(year, day))
            .unwrap()
    }};
    (example) => {{
        let (year, day) = $crate::aoc::day::parse_year_and_day_from_module(module_path!());
        let example_path = format!("input/{}/example/day{}.txt", year, day);
        &mut $crate::aoc::input::read_input(&std::path::PathBuf::from(example_path)).unwrap()
    }};
}
