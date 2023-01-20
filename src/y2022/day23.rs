use std::{collections::HashSet, io::BufRead};

use glam::IVec2;
use itertools::Itertools;

#[cfg(feature = "parallel")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::{
        grid::{
            self,
            directions::{
                Directions, BOTTOM_LEFT, BOTTOM_RIGHT, DOWN, LEFT, RIGHT, TOP_LEFT, TOP_RIGHT, UP,
            },
            minmax::minmax_ivec2,
        },
        parsing::lines_iter,
    },
};

pub const TITLE: &str = "Unstable Diffusion";

const DIRECTIONS: [[IVec2; 3]; 4] = [
    [TOP_LEFT, UP, TOP_RIGHT],
    [BOTTOM_LEFT, DOWN, BOTTOM_RIGHT],
    [TOP_LEFT, LEFT, BOTTOM_LEFT],
    [TOP_RIGHT, RIGHT, BOTTOM_RIGHT],
];

pub struct Solution {
    elves: HashSet<IVec2>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let elves = grid::parse_set(lines_iter(input));
        Self { elves }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let mut elves = self.elves.clone();

        for round in 0..10 {
            move_round(&mut elves, round);
        }

        let (min, max) = minmax_ivec2(elves.iter());
        let size = (max.x - min.x + 1) * (max.y - min.y + 1);
        (size - elves.len() as i32).to_string()
    }

    fn part2(&self) -> String {
        let mut elves = self.elves.clone();
        for round in 0.. {
            if move_round(&mut elves, round) == 0 {
                return (round + 1).to_string();
            }
        }
        unreachable!();
    }
}

fn move_round(elves: &mut HashSet<IVec2>, round: u16) -> u16 {
    #[cfg(feature = "parallel")]
    let elves_iter = elves.par_iter();
    #[cfg(not(feature = "parallel"))]
    let elves_iter = elves.iter();

    let preferred_targets = elves_iter
        .filter_map(|elf| get_prefered_target(elves, elf, round).map(|target| (*elf, target)))
        .collect::<Vec<(IVec2, IVec2)>>();

    let mut moves = 0;
    preferred_targets
        .iter()
        .into_group_map_by(|(_, target)| *target)
        .into_iter()
        .for_each(|(target, from)| {
            if from.len() == 1 {
                elves.remove(&from[0].0);
                elves.insert(target);
                moves += 1;
            }
        });
    moves
}

fn get_prefered_target(elves: &HashSet<IVec2>, elf: &IVec2, round: u16) -> Option<IVec2> {
    if elf.neighbors().iter().all(|n| !elves.contains(n)) {
        None
    } else {
        (0..4).into_iter().find_map(|offset| {
            let direction = DIRECTIONS[(round + offset) as usize % 4];
            let checks = direction.map(|d| *elf + d);
            if checks.iter().all(|n| !elves.contains(n)) {
                Some(checks[1])
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 23));
        assert_eq!(day.part1(), "110");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 23));
        assert_eq!(day.part1(), "4005");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 23));
        assert_eq!(day.part2(), "20");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 23));
        assert_eq!(day.part2(), "1008");
    }
}
