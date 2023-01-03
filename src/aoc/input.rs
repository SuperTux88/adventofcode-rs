use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use dirs::{cache_dir, config_dir, home_dir};
use reqwest::{
    blocking::Client,
    header::{COOKIE, USER_AGENT},
};

const AOC_SESSION_ENV_VAR: &str = "ADVENT_OF_CODE_SESSION";

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
        get_cache_path_and_download_if_needed(year, day)
    } else {
        match get_input_subpath(year, day) {
            path if path.exists() => Ok(path),
            path => get_cache_path_and_download_if_needed(year, day).map_err(|error| {
                format!(
                    "Input file ({}) doesn't exist and error downloading input: {}",
                    path.as_path().display(),
                    error
                )
            }),
        }
    }
}

/// Returns the AoC session cookie string.
/// The session cookie is read from the environment variable `ADVENT_OF_CODE_SESSION` or from the
/// file `~/.adventofcode.session` or `~/.config/adventofcode.session`.
fn get_aoc_session() -> Result<String, String> {
    match env::var(AOC_SESSION_ENV_VAR) {
        Ok(session) => Ok(session),
        Err(_) => {
            let session_files = vec![
                home_dir().map(|h| h.join(".adventofcode.session")),
                config_dir().map(|h| h.join("adventofcode.session")),
            ];

            let session_file =
                session_files
                    .iter()
                    .flatten()
                    .find(|f| f.exists())
                    .ok_or(format!(
                        "No AoC session found, set ${} environment variable or create {}",
                        AOC_SESSION_ENV_VAR,
                        session_files
                            .iter()
                            .flatten()
                            .map(|f| f.display().to_string())
                            .collect::<Vec<String>>()
                            .join(" or ")
                    ))?;

            Ok(std::fs::read_to_string(session_file).map_err(|error| {
                format!(
                    "Error reading session file ({}): {}",
                    session_file.as_path().display(),
                    error
                )
            })?)
        }
    }
}

/// Downloads the input for the given year and day from adventofcode.com to the given path.
fn download_input(year: u16, day: u8, input_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = Client::builder().gzip(true).build().unwrap();
    let mut response = client
        .get(url)
        .header(COOKIE, format!("session={}", get_aoc_session()?.trim()))
        .header(
            USER_AGENT,
            "AoC solutions at github.com/SuperTux88/adventofcode-rs",
        )
        .send()?
        .error_for_status()?;

    let mut file = std::fs::File::create(input_path).map_err(|error| {
        format!(
            "Error creating input file ({}): {}",
            input_path.as_path().display(),
            error
        )
    })?;
    response.copy_to(&mut file)?;

    Ok(())
}

/// Gets the path to the input file for the given year and day, downloading it if it doesn't exist.
fn get_cache_path_and_download_if_needed(year: u16, day: u8) -> Result<PathBuf, String> {
    let input_path = cache_dir()
        .map(|d| d.join("adventofcode").join(get_input_subpath(year, day)))
        .ok_or("Error getting cache directory")?;
    if let Some(parent) = input_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            format!(
                "Error creating input cache directory ({}): {}",
                parent.display(),
                error
            )
        })?;
    }

    if !input_path.exists() {
        download_input(year, day, &input_path).map_err(|error| {
            format!(
                "Error downloading input file ({}): {}",
                input_path.as_path().display(),
                error
            )
        })?;
    }

    Ok(input_path)
}

#[macro_export]
macro_rules! input {
    (input: $year:tt $day:tt) => {
        &mut $crate::aoc::input::read_input(&$crate::aoc::input::get_input_subpath($year, $day))
            .unwrap()
    };
    (example: $year:tt $day:tt) => {{
        let example_path = format!("input/{}/example/day{}.txt", $year, $day);
        &mut $crate::aoc::input::read_input(&std::path::PathBuf::from(example_path)).unwrap()
    }};
}
