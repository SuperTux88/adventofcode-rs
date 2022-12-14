use adventofcode::*;

fn main() -> Result<(), String> {
    let mut input = input::read_input(input::get_default_input_path(2022, 1))?;
    run_solution_for(2022, 1, &mut input)
}
