use std::{collections::HashMap, io::BufRead};

use itertools::Itertools;

#[cfg(feature = "parallel")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
#[cfg(not(feature = "parallel"))]
use std::slice::Iter;

use crate::{
    aoc::day::{DayParser, DaySolution},
    common::parsing::lines_iter,
};

pub const TITLE: &str = "Hot Springs";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    springs: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

pub struct Solution {
    records: Vec<Record>,
}

impl DayParser for Solution {
    fn with_input(input: &mut dyn BufRead) -> Self {
        let records = lines_iter(input)
            .map(|line| {
                let (springs, damaged_groups) = line.split_once(' ').unwrap();
                let springs = springs
                    .chars()
                    .map(|c| match c {
                        '.' => Condition::Operational,
                        '#' => Condition::Damaged,
                        '?' => Condition::Unknown,
                        _ => panic!("Invalid spring condition: {}", c),
                    })
                    .collect();

                let damaged_groups = damaged_groups
                    .split(',')
                    .map(|i| i.parse().unwrap())
                    .collect();

                Record {
                    springs,
                    damaged_groups,
                }
            })
            .collect_vec();

        Self { records }
    }
}

impl DaySolution for Solution {
    fn part1(&self) -> String {
        self.records_iter()
            .map(|record| record.valid_arrangements())
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.records_iter()
            .map(|record| record.unfold(5).valid_arrangements())
            .sum::<u64>()
            .to_string()
    }
}

impl Solution {
    #[cfg(not(feature = "parallel"))]
    fn records_iter(&self) -> Iter<'_, Record> {
        self.records.iter()
    }

    #[cfg(feature = "parallel")]
    fn records_iter(&self) -> impl ParallelIterator<Item = &Record> {
        self.records.par_iter()
    }
}

impl Record {
    fn valid_arrangements(&self) -> u64 {
        valid_arrangements_cached(&mut HashMap::new(), &self.springs, &self.damaged_groups)
    }

    fn unfold(&self, times: usize) -> Record {
        let springs = self
            .springs
            .iter()
            .copied()
            .chain([Condition::Unknown])
            .cycle()
            .take(self.springs.len() * times + (times - 1))
            .collect_vec();

        Record {
            springs,
            damaged_groups: self.damaged_groups.repeat(times),
        }
    }
}

fn valid_arrangements_cached(
    cache: &mut HashMap<(usize, usize), u64>,
    springs: &[Condition],
    damaged_groups: &[usize],
) -> u64 {
    cache
        .get(&(springs.len(), damaged_groups.len()))
        .copied()
        .unwrap_or_else(|| {
            let count = match (springs.split_first(), damaged_groups.split_first()) {
                (_, None) if springs.contains(&Condition::Damaged) => 0, // invalid if still damaged springs, but no groups left
                (_, None) => 1, // valid if no groups left, and no damaged springs left
                (None, Some(_)) => 0, // invalid if no springs left but still groups required
                (Some((Condition::Operational, tail)), _) => {
                    // skip operational springs
                    valid_arrangements_cached(cache, tail, damaged_groups)
                }
                (Some((Condition::Damaged, _)), Some((current_group, remaining_groups))) => {
                    if &springs.len() >= current_group {
                        // try to consume all damaged springs in the current group
                        let (group, tail) = springs.split_at(*current_group);
                        if group.contains(&Condition::Operational) {
                            0 // invalid if there are still operational springs in the group
                        } else {
                            match tail.split_first() {
                                None => {
                                    if remaining_groups.is_empty() {
                                        1 // valid if no more springs left, and no more groups required
                                    } else {
                                        0 // invalid if no more springs left, but still groups required
                                    }
                                }
                                Some((Condition::Damaged, _)) => 0, // invalid if the next spring is also damaged
                                Some((_, tail)) => {
                                    // otherwise skip one spring and continue
                                    valid_arrangements_cached(cache, tail, remaining_groups)
                                }
                            }
                        }
                    } else {
                        0 // invalid if there are not enough springs left
                    }
                }
                (Some((Condition::Unknown, tail)), _) => {
                    let mut springs = tail.to_vec();

                    // adding an operational spring is not needed, as they are skipped anyway
                    let operational = valid_arrangements_cached(cache, &springs, damaged_groups);

                    springs.insert(0, Condition::Damaged);
                    let damaged = valid_arrangements_cached(cache, &springs, damaged_groups);

                    operational + damaged
                }
            };
            cache.insert((springs.len(), damaged_groups.len()), count);
            count
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part1(), "21");
    }

    #[test]
    fn test_part1_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part1(), "7118");
    }

    #[test]
    fn test_part2_example() {
        let solution = Solution::with_input(input!(example));
        assert_eq!(solution.part2(), "525152");
    }

    #[test]
    fn test_part2_input() {
        let solution = Solution::with_input(input!(input));
        assert_eq!(solution.part2(), "7030194981795");
    }
}
