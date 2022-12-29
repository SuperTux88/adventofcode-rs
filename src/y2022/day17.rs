use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use glam::IVec2;

use crate::{
    common::grid::{directions::Direction, parse_set, walk::Walk},
    day::AoCDay,
};

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

pub struct Solution {
    rocks: Vec<Vec<IVec2>>,
    jets: Vec<Direction>,
}

impl AoCDay for Solution {
    fn title() -> &'static str {
        "Pyroclastic Flow"
    }

    fn with_input(input: &mut impl BufRead) -> Self {
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

    fn part1(&self) -> String {
        let mut chamber = (0..7).map(|x| IVec2::new(x, 1)).collect::<HashSet<_>>();
        let mut direction_offset = 0;

        for r in 0..2022 {
            direction_offset = self.fall_rock(
                &mut chamber,
                &self.rocks[r % self.rocks.len()],
                direction_offset,
            );
        }

        chamber_height(&chamber).to_string()
    }

    fn part2(&self) -> String {
        todo!();
    }
}

impl Solution {
    fn fall_rock(
        &self,
        chamber: &mut HashSet<IVec2>,
        rock: &[IVec2],
        mut direction_offset: u16,
    ) -> u16 {
        let mut rock = rock
            .iter()
            .map(|r| r.move_distance(Direction::Up, chamber_height(chamber) + 3))
            .collect::<Vec<_>>();

        loop {
            let moved_rock = rock
                .iter()
                .map(|r| r.move_step(self.jets[direction_offset as usize % self.jets.len()]))
                .collect::<Vec<_>>();
            if !moved_rock
                .iter()
                .any(|r| r.x < 0 || r.x > 6 || chamber.contains(r))
            {
                rock = moved_rock;
            }
            direction_offset += 1;

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
    #[ignore]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 17));
        assert_eq!(day.part2(), "1514285714288");
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 17));
        assert_eq!(day.part2(), "1565517241382");
    }
}
