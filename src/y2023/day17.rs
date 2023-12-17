use std::{collections::HashMap, io::BufRead};

use colored::Colorize;
use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::astar;

use crate::{
    aoc::{
        day::{DayParser, DaySolution},
        output,
    },
    common::{
        grid::{
            directions::{Direction, Turn},
            distance::ManhattenDistance,
            minmax, parse_map, print_area,
            walk::Walk,
        },
        parsing::lines_iter,
    },
};

pub const TITLE: &str = "Clumsy Crucible";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: IVec2,
    dir: Direction,
    steps: u8,
}

const START: State = State {
    pos: IVec2::ZERO,
    dir: Direction::Right,
    steps: 0,
};

pub struct Solution {
    map: HashMap<IVec2, u8>,
    target: IVec2,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let map = parse_map(lines_iter(input), |c| c.to_digit(10).unwrap() as u8);
        let (_, target) = minmax::minmax_ivec2(map.keys());
        Self { map, target }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.find_path(1, 3).to_string()
    }

    fn part2(&self) -> String {
        self.find_path(4, 10).to_string()
    }
}

impl Solution {
    fn find_path(&self, min_steps: u8, max_steps: u8) -> u32 {
        let path = astar(
            &START,
            |state| state.next(&self.map, min_steps, max_steps),
            |state| state.pos.manhattan_distance(&self.target),
            |state| state.pos == self.target && state.steps >= min_steps,
        )
        .expect("No path found");

        if output::is_debug_enabled() {
            self.print_path(&path.0);
        }

        path.1
    }

    fn print_path(&self, path: &[State]) {
        let path_direction = path
            .iter()
            .map(|state| (state.pos, state.dir))
            .collect::<HashMap<IVec2, Direction>>();

        print_area((IVec2::ZERO, self.target), |pos| {
            match path_direction.get(&pos) {
                Some(Direction::Up) => "^".red(),
                Some(Direction::Right) => ">".red(),
                Some(Direction::Down) => "v".red(),
                Some(Direction::Left) => "<".red(),
                None => self.map.get(&pos).unwrap().to_string().normal(),
            }
            .to_string()
        });
    }
}

impl State {
    fn next(&self, map: &HashMap<IVec2, u8>, min_steps: u8, max_steps: u8) -> Vec<(Self, u32)> {
        let mut possible_directions = vec![];
        if self.steps < max_steps {
            possible_directions.push((self.dir, self.steps + 1));
        }
        if self.steps >= min_steps {
            possible_directions.push((self.dir.turn(&Turn::Left), 1));
            possible_directions.push((self.dir.turn(&Turn::Right), 1));
        }

        possible_directions
            .into_iter()
            .flat_map(|(dir, steps)| {
                let pos = self.pos.move_step(dir);
                map.get(&pos)
                    .map(|&value| (Self { pos, dir, steps }, value as u32))
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "102");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "791");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "94");
    }

    #[test]
    fn test_part2_example2() {
        let solution = Solution::with_input(input!(example, 2));
        assert_eq!(solution.part2(), "71");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "900");
    }
}
