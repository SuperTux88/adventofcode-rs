use std::io::BufRead;

pub struct Day {
    pub year: u16,
    pub day: u8,
    pub title: &'static str,

    parse: fn(&mut dyn BufRead) -> Box<dyn DaySolution>,
}

impl Day {
    pub fn new(
        year: u16,
        day: u8,
        title: &'static str,
        parse: fn(&mut dyn BufRead) -> Box<dyn DaySolution>,
    ) -> Self {
        Self {
            year,
            day,
            title,
            parse,
        }
    }

    pub fn parse(&self, input: &mut dyn BufRead) -> Box<dyn DaySolution> {
        (self.parse)(input)
    }
}

pub trait DayParser {
    fn with_input(input: &mut dyn BufRead) -> Self;
}

pub trait DaySolution {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

/// Parses the year and day from the given module name (for example with `module_path!()`)
///
/// For example, "adventofcode::y2022::day1::tests" will return (2022, 1).
pub fn parse_year_and_day_from_module(module: &str) -> (u16, u8) {
    let mut parts = module.split("::");
    if parts.next() != Some("adventofcode") {
        panic!("Invalid module name: {}", module);
    }
    let year = parts.next().unwrap()[1..].parse().unwrap();
    let day = parts.next().unwrap()[3..].parse().unwrap();
    (year, day)
}
