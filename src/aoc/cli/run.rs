use std::path::Path;

use crate::aoc::{day::Day, part::Part};

pub type RunFunction = fn(day: Day, part: &Part, input_path: &Path) -> Result<(), String>;

pub trait Run {
    fn run(day: Day, part: &Part, input_path: &Path) -> Result<(), String>;
}
