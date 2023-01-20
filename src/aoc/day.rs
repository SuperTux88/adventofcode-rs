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
