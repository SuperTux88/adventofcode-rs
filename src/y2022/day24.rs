use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use glam::IVec2;
use num::integer::lcm;
use pathfinding::prelude::astar;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::{
        grid::{
            directions::{Direction, Directions},
            distance::ManhattenDistance,
            minmax::minmax_ivec2,
            parse_map,
        },
        parsing::lines_iter,
    },
};

pub const TITLE: &str = "Blizzard Basin";

#[derive(Debug)]
enum Tile {
    Free,
    Wall,
    Blizzard(Direction),
}

#[derive(Debug, Clone)]
struct Blizzard {
    pos: IVec2,
    dir: Direction,
}

pub struct Solution {
    start: IVec2,
    end: IVec2,
    blocked_at_times: Vec<HashSet<IVec2>>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let map = parse_map(lines_iter(input), |c| match c {
            '.' => Tile::Free,
            '#' => Tile::Wall,
            d @ ('^' | '>' | 'v' | '<') => Tile::Blizzard(d.into()),
            _ => panic!("Invalid tile: {}", c),
        });
        let (min, max) = minmax_ivec2(map.keys());
        let start = free_in_line(&map, &min.y);
        let end = free_in_line(&map, &max.y);

        let mut walls = map
            .iter()
            .filter_map(|(pos, tile)| match tile {
                Tile::Wall => Some(pos),
                _ => None,
            })
            .cloned()
            .collect::<HashSet<_>>();
        walls.insert(start.up());
        walls.insert(end.down());

        let blizzards = map
            .into_iter()
            .filter_map(|(pos, tile)| match tile {
                Tile::Blizzard(dir) => Some(Blizzard { pos, dir }),
                _ => None,
            })
            .collect::<Vec<_>>();

        let blizzard_loop = lcm(max.x - 1, max.y - 1);
        let blocked_at_times = (0..blizzard_loop)
            .scan(blizzards, |state, _| {
                let current = state.clone();
                state.iter_mut().for_each(|b| match b.dir {
                    Direction::Up => {
                        if b.pos.y == 1 {
                            b.pos.y = max.y - 1;
                        } else {
                            b.pos.y -= 1;
                        }
                    }
                    Direction::Right => {
                        if b.pos.x == max.x - 1 {
                            b.pos.x = 1;
                        } else {
                            b.pos.x += 1;
                        }
                    }
                    Direction::Down => {
                        if b.pos.y == max.y - 1 {
                            b.pos.y = 1;
                        } else {
                            b.pos.y += 1;
                        }
                    }
                    Direction::Left => {
                        if b.pos.x == 1 {
                            b.pos.x = max.x - 1;
                        } else {
                            b.pos.x -= 1;
                        }
                    }
                });
                Some(current)
            })
            .map(|blizzards| {
                let mut blizzards = blizzards.into_iter().map(|b| b.pos).collect::<HashSet<_>>();
                blizzards.extend(&walls);
                blizzards
            })
            .collect::<Vec<_>>();

        Self {
            start,
            end,
            blocked_at_times,
        }
    }
}

fn free_in_line(map: &HashMap<IVec2, Tile>, line: &i32) -> IVec2 {
    map.iter()
        .find_map(|(pos, tile)| match tile {
            Tile::Free if &pos.y == line => Some(pos),
            _ => None,
        })
        .unwrap()
        .to_owned()
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        let start = State::new(self.start);
        self.find_path(&start, &self.end).0.to_string()
    }

    fn part2(&self) -> String {
        let start = State::new(self.start);
        let (dist1, state) = self.find_path(&start, &self.end);
        let (dist2, state) = self.find_path(&state, &self.start);
        let (dist3, _) = self.find_path(&state, &self.end);
        (dist1 + dist2 + dist3).to_string()
    }
}

impl Solution {
    fn find_path(&self, start: &State, end: &IVec2) -> (u32, State) {
        let res = astar(
            start,
            |state| state.next(&self.blocked_at_times),
            |state| state.pos.manhattan_distance(end),
            |state| &state.pos == end,
        )
        .expect("No path found");
        (res.1, res.0.last().unwrap().clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: IVec2,
    time: usize,
}

impl State {
    fn new(pos: IVec2) -> Self {
        Self { pos, time: 0 }
    }

    fn next(&self, blocked_at_times: &[HashSet<IVec2>]) -> Vec<(Self, u32)> {
        let next_time = (self.time + 1) % blocked_at_times.len();
        let mut next = self.pos.directions();
        next.push(self.pos);
        next.retain(|pos| !blocked_at_times[next_time].contains(pos));
        next.into_iter()
            .map(|pos| {
                (
                    Self {
                        pos,
                        time: next_time,
                    },
                    1,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 24));
        assert_eq!(day.part1(), "18");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 24));
        assert_eq!(day.part1(), "245");
    }

    #[test]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 24));
        assert_eq!(day.part2(), "54");
    }

    #[test]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 24));
        assert_eq!(day.part2(), "798");
    }
}
