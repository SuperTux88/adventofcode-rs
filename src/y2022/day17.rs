use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use glam::IVec2;
use itertools::Itertools;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::grid::{directions::Direction, parse_set, walk::Walk},
};

pub const TITLE: &str = "Pyroclastic Flow";

const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

const ROCKS_P1: u16 = 2022;
const ROCKS_P2: u64 = 1_000_000_000_000;

pub struct Solution {
    rocks: Vec<Vec<IVec2>>,
    jets: Vec<Direction>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let rocks = ROCKS
            .split("\n\n")
            .map(|rock_str| {
                let rock = parse_set(rock_str.lines().map(|l| l.to_string()));
                rock.iter()
                    .map(|r| {
                        r.move_distance(Direction::Up, rock.iter().max_by_key(|r| r.y).unwrap().y)
                            .move_distance(Direction::Right, 2)
                    })
                    .collect()
            })
            .collect();

        let jets = io::read_to_string(input)
            .unwrap()
            .trim()
            .chars()
            .map(|c| c.into())
            .collect();

        Self { rocks, jets }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let mut chamber = (0..7).map(|x| IVec2::new(x, 1)).collect::<HashSet<_>>();
        let mut direction_offset = 0;

        for r in 0..ROCKS_P1 {
            direction_offset = self.fall_rock(
                &mut chamber,
                &self.rocks[r as usize % self.rocks.len()],
                direction_offset,
            );
        }

        chamber_height(&chamber).to_string()
    }

    fn part2(&self) -> String {
        let mut chamber = (0..7).map(|x| IVec2::new(x, 1)).collect::<HashSet<_>>();
        let mut rock_offset = 0;
        let mut direction_offset = 0;

        let mut states = vec![(vec![0; 7], rock_offset, direction_offset)];
        let mut heights = vec![0];

        let loop_state = loop {
            direction_offset =
                self.fall_rock(&mut chamber, &self.rocks[rock_offset], direction_offset);
            rock_offset = (rock_offset + 1) % self.rocks.len();

            let state = (top_line(&chamber), rock_offset, direction_offset);
            if states.contains(&state) {
                break state;
            } else {
                states.push(state);
                heights.push(chamber_height(&chamber));
            }
        };

        let loop_start = states.iter().position(|s| s == &loop_state).unwrap();
        let loop_size = states.len() - loop_start;

        let after_loop_height = chamber_height(&chamber);
        let loop_height = after_loop_height - heights[loop_start];

        let top_line_after_loop = loop_state.0;
        let top_line_offset = top_line_after_loop.iter().min().unwrap();
        let mut end_chamber = top_line_after_loop
            .iter()
            .enumerate()
            .map(|(x, y)| IVec2::new(x as i32, *y - top_line_offset + 1))
            .collect::<HashSet<_>>();

        let remaining_rocks = ROCKS_P2 - states.len() as u64;
        for _ in 0..(remaining_rocks % loop_size as u64) {
            direction_offset =
                self.fall_rock(&mut end_chamber, &self.rocks[rock_offset], direction_offset);
            rock_offset = (rock_offset + 1) % self.rocks.len();
        }

        (after_loop_height as u64
            + remaining_rocks / loop_size as u64 * loop_height as u64
            + chamber_height(&end_chamber) as u64)
            .to_string()
    }
}

impl Solution {
    fn fall_rock(
        &self,
        chamber: &mut HashSet<IVec2>,
        rock: &[IVec2],
        mut direction_offset: usize,
    ) -> usize {
        let mut rock = rock
            .iter()
            .map(|r| r.move_distance(Direction::Up, chamber_height(chamber) + 3))
            .collect::<Vec<_>>();

        loop {
            let moved_rock = rock
                .iter()
                .map(|r| r.move_step(self.jets[direction_offset]))
                .collect::<Vec<_>>();
            if !moved_rock
                .iter()
                .any(|r| r.x < 0 || r.x > 6 || chamber.contains(r))
            {
                rock = moved_rock;
            }
            direction_offset = (direction_offset + 1) % self.jets.len();

            let fallen_rock = rock
                .iter()
                .map(|r| r.move_step(Direction::Down))
                .collect::<Vec<_>>();

            if fallen_rock.iter().any(|r| chamber.contains(r)) {
                chamber.extend(rock);
                break direction_offset;
            } else {
                rock = fallen_rock;
            }
        }
    }
}

fn chamber_height(chamber: &HashSet<IVec2>) -> i32 {
    -(chamber.iter().min_by_key(|r| r.y).unwrap().y - 1)
}

fn top_line(chamber: &HashSet<IVec2>) -> Vec<i32> {
    let top_line = chamber
        .iter()
        .into_grouping_map_by(|r| r.x)
        .min_by_key(|_, r| r.y)
        .iter()
        .sorted_by_key(|(x, _)| *x)
        .map(|(_, r)| r.y)
        .collect::<Vec<_>>();
    let offset = top_line.iter().max().unwrap();
    top_line.iter().map(|y| y - offset).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 17));
        assert_eq!(day.part1(), "3068");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 17));
        assert_eq!(day.part1(), "3177");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 17));
        assert_eq!(day.part2(), "1514285714288");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 17));
        assert_eq!(day.part2(), "1565517241382");
    }
}
