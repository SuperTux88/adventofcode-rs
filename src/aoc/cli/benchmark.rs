use std::{
    env,
    path::Path,
    time::{Duration, Instant},
};

use colored::Colorize;

use crate::aoc::{
    day::{Day, DaySolution},
    input,
    part::Part,
};

use super::run::Run;

const AOC_BENCH_LOOPS: u16 = 10;
const AOC_BENCH_LOOPS_ENV_VAR: &str = "AOC_BENCH_LOOPS";

pub struct Benchmark {}

impl Run for Benchmark {
    fn run(day: Day, part: &Part, input_path: &Path) -> Result<(), String> {
        print!("{} day {:>2}: ", day.year, day.day);

        let times = collect_times(day, input_path, part)?;

        match part {
            Part::Both => println!(
                "{} {} | {} {} | {} {} | {} {}",
                "parsing:".white(),
                format_times(times.parsing),
                "part 1:".white(),
                format_times(times.part1),
                "part 2:".white(),
                format_times(times.part2),
                "total:".white(),
                format_times(times.total)
            ),
            part => {
                let part_times = match part {
                    Part::Part1 => times.part1,
                    Part::Part2 => times.part2,
                    Part::Both => unreachable!(),
                };
                println!(
                    "{} {} | {} {} | {} {}",
                    "parsing:".white(),
                    format_times(times.parsing),
                    format!("part {}:", part).white(),
                    format_times(part_times),
                    "total:".white(),
                    format_times(times.total)
                )
            }
        };

        Ok(())
    }
}

#[derive(Default)]
struct Times {
    parsing: Vec<Duration>,
    part1: Vec<Duration>,
    part2: Vec<Duration>,
    total: Vec<Duration>,
}

fn collect_times(day: Day, input_path: &Path, part: &Part) -> Result<Times, String> {
    let mut times = Times::default();

    let loops = env::var(AOC_BENCH_LOOPS_ENV_VAR)
        .map(|v| v.parse::<u16>().unwrap_or(AOC_BENCH_LOOPS))
        .unwrap_or(AOC_BENCH_LOOPS);
    for _i in 0..loops {
        let start = std::time::Instant::now();
        let solution = day.parse(&mut input::read_input(input_path)?);
        times.parsing.push(start.elapsed());

        if part == &Part::Part1 || part == &Part::Both {
            times.part1.push(run_part(solution.as_ref(), &Part::Part1));
        }
        if part == &Part::Part2 || part == &Part::Both {
            times.part2.push(run_part(solution.as_ref(), &Part::Part2));
        }

        times.total.push(start.elapsed());
    }

    Ok(times)
}

fn run_part(day: &dyn DaySolution, part: &Part) -> Duration {
    let start = Instant::now();
    part.run_for(day);
    start.elapsed()
}

fn format_times(times: Vec<Duration>) -> String {
    let avg = times.iter().sum::<Duration>() / times.len() as u32;
    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();
    format!(
        "({} / {} / {})",
        format_time(min).green(),
        format_time(&avg).yellow(),
        format_time(max).red()
    )
}

fn format_time(time: &Duration) -> String {
    format!("{:>8.2?}", time)
}
