use std::{collections::HashSet, io::BufRead};

use glam::IVec2;
use itertools::Itertools;

use crate::{
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
    day::AoCDay,
};

const DIRECTIONS: [[IVec2; 3]; 4] = [
    [TOP_LEFT, UP, TOP_RIGHT],
    [BOTTOM_LEFT, DOWN, BOTTOM_RIGHT],
    [TOP_LEFT, LEFT, BOTTOM_LEFT],
    [TOP_RIGHT, RIGHT, BOTTOM_RIGHT],
];

pub struct Solution {
    elves: HashSet<IVec2>,
}

impl AoCDay for Solution {
    fn title() -> &'static str {
        "Unstable Diffusion"
    }

    fn with_input(input: &mut impl BufRead) -> Self {
        let elves = grid::parse_set(lines_iter(input));
        Self { elves }
    }

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
        todo!();
    }
}

fn move_round(elves: &mut HashSet<IVec2>, round: u16) {
    let preferred_targets = elves
        .iter()
        .filter_map(|elf| get_prefered_target(elves, elf, round).map(|target| (*elf, target)));

    preferred_targets
        .into_group_map_by(|(_, target)| *target)
        .into_iter()
        .for_each(|(target, from)| {
            if from.len() == 1 {
                elves.remove(&from[0].0);
                elves.insert(target);
            }
        });
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
    #[ignore]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 23));
        assert_eq!(day.part2(), "20");
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 23));
        assert_eq!(day.part2(), "1008");
    }
}
