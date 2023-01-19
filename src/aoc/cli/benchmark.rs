use std::{
    env,
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{
    aoc::{day::DaySolution, input, part::Part, run},
    Solutions,
};

const AOC_BENCH_LOOPS: u16 = 10;
const AOC_BENCH_LOOPS_ENV_VAR: &str = "AOC_BENCH_LOOPS";

#[derive(Default)]
struct Times {
    parsing: Vec<Duration>,
    part1: Vec<Duration>,
    part2: Vec<Duration>,
    total: Vec<Duration>,
}

pub fn run_benchmark(year: u16, day: u8, part: &Part, path: &PathBuf) -> Result<(), String> {
    print!("{} day {:>2}: ", year, day);

    let times = collect_times(year, day, path, part)?;

    match part {
        Part::Both => println!(
            "parsing: {} | part 1: {} | part 2: {} | total: {}",
            format_times(times.parsing),
            format_times(times.part1),
            format_times(times.part2),
            format_times(times.total)
        ),
        part => {
            let part_times = match part {
                Part::Part1 => times.part1,
                Part::Part2 => times.part2,
                Part::Both => unreachable!(),
            };
            println!(
                "parsing: {} | part {}: {} | total: {}",
                format_times(times.parsing),
                part,
                format_times(part_times),
                format_times(times.total)
            )
        }
    };

    Ok(())
}

fn collect_times(year: u16, day: u8, input_path: &PathBuf, part: &Part) -> Result<Times, String> {
    let mut times = Times::default();

    let loops = env::var(AOC_BENCH_LOOPS_ENV_VAR)
        .map(|v| v.parse::<u16>().unwrap_or(AOC_BENCH_LOOPS))
        .unwrap_or(AOC_BENCH_LOOPS);
    for _i in 0..loops {
        let start = std::time::Instant::now();
        let solution = Solutions::get_with_input(year, day, &mut input::read_input(input_path)?);
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
    run::run_part(day, part);
    start.elapsed()
}

fn format_times(times: Vec<Duration>) -> String {
    let avg = times.iter().sum::<Duration>() / times.len() as u32;
    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();
    format!("({:>8.2?} / {:>8.2?} / {:>8.2?})", min, avg, max)
}
