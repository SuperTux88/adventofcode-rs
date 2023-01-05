use itertools::Itertools;

use console_error_panic_hook::set_once as set_panic_hook;

use adventofcode::{
    aoc::{output, Part},
    Solutions,
};

#[cfg(all(feature = "online", target_arch = "wasm32"))]
compile_error!("feature \"online\" cannot be enabled with \"wasm\" at the same time. Use \"...  --no-default-features --features wasm\" instead.");
#[cfg(all(feature = "parallel", target_arch = "wasm32"))]
compile_error!("feature \"parallel\" cannot be enabled with \"wasm\" at the same time. Use \"...  --no-default-features --features wasm\" instead.");

pub fn main() {
    set_panic_hook();

    output::println("Advent of Code WASM".to_string());

    output::println(Solutions::years().iter().join(", "));
    output::println(Solutions::days_for_year(2022).iter().join(", "));

    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    let results = Solutions::run(2022, 1, &Part::Both, &mut input.as_bytes());
    output::println(format!("Part 1: {:?}", results.part1));
    output::println(format!("Part 2: {:?}", results.part2));
}
