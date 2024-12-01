use criterion::{criterion_group, criterion_main, Criterion};

use adventofcode::aoc::{
    day::{DayParser, DaySolution},
    input, output,
};

const AOC_BENCH_DOWNLOAD_INPUT_ENV_VAR: &str = "AOC_BENCH_DOWNLOAD_INPUT";

macro_rules! benchmark {
    ($year: ident, $day: ident) => {
        fn $day(c: &mut Criterion) {
            let year = stringify!($year)[1..].parse::<u16>().unwrap();
            let day = stringify!($day)[3..].parse::<u8>().unwrap();
            let download_input = std::env::var(AOC_BENCH_DOWNLOAD_INPUT_ENV_VAR).is_ok();
            let input_path = input::get_default_input_path(year, day, download_input).unwrap();
            output::disable_output();

            c.bench_function(format!("{} day {} parsing", year, day).as_str(), |b| {
                b.iter(|| {
                    let mut input = input::read_input(&input_path).unwrap();
                    <$year::$day::Solution as DayParser>::with_input(&mut input)
                })
            });

            c.bench_function(format!("{} day {} part 1", year, day).as_str(), |b| {
                let mut input = input::read_input(&input_path).unwrap();
                let solution = <$year::$day::Solution as DayParser>::with_input(&mut input);

                b.iter(|| solution.part1())
            });

            c.bench_function(format!("{} day {} part 2", year, day).as_str(), |b| {
                let mut input = input::read_input(&input_path).unwrap();
                let solution = <$year::$day::Solution as DayParser>::with_input(&mut input);

                b.iter(|| solution.part2())
            });
        }
    };
}

macro_rules! benchmarks {
    ($(($name:ident / $year:ident: $($day:ident),+)),+$(,)?) => {
        $(
            use adventofcode::$year;

            struct $name {}
            impl $name {
                $(
                    benchmark!($year, $day);
                )+
            }
        )+

        criterion_group!(benchmarks, $($($name::$day,)+)+);
        criterion_main!(benchmarks);
    };
}

#[rustfmt::skip]
benchmarks!(
    (Benchmark2016 / y2016: day1),
    (Benchmark2021 / y2021: day1),
    (Benchmark2022 / y2022: day1, day3, day4, day6, day10, day13, day16, day17, day23, day24, day25),
    (Benchmark2023 / y2023: day1, day2, day3, day4, day6, day7, day8, day9, day11, day12, day13, day15, day17, day18),
    (Benchmark2024 / y2024: day1),
);
