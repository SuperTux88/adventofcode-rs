use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

use crate::output;

pub mod area;
pub mod directions;
pub mod distance;
pub mod minmax;
pub mod walk;

pub fn parse_set(lines: impl Iterator<Item = String>) -> HashSet<IVec2> {
    lines
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '#' => Some(IVec2::new(x as i32, y as i32)),
                    _ => None,
                })
                .collect::<HashSet<IVec2>>()
        })
        .collect()
}

pub fn parse_map<T>(
    lines: impl Iterator<Item = String>,
    parse_char: fn(char) -> T,
) -> HashMap<IVec2, T> {
    lines
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), parse_char(c)))
                .collect::<HashMap<IVec2, T>>()
        })
        .collect()
}

pub fn parse_vec<T>(lines: impl Iterator<Item = String>, parse_char: fn(char) -> T) -> Vec<Vec<T>> {
    lines
        .map(move |line| line.chars().map(parse_char).collect_vec())
        .collect_vec()
}

pub fn print_set(set: &HashSet<IVec2>) {
    let (min, max) = minmax::minmax_ivec2(set.iter());
    print_set_range(set, (min, max));
}
pub fn print_set_range(set: &HashSet<IVec2>, range: (IVec2, IVec2)) {
    if output::is_debug_enabled() {
        for y in range.0.y..=range.1.y {
            output::println_debug(
                (range.0.x..=range.1.x)
                    .map(|x| {
                        if set.contains(&IVec2::new(x, y)) {
                            '#'
                        } else {
                            ' '
                        }
                    })
                    .collect(),
            );
        }
    }
}

pub fn print_area<F>(area: (IVec2, IVec2), map_pos: F)
where
    F: Fn(IVec2) -> String,
{
    if output::is_debug_enabled() {
        for y in area.0.y..=area.1.y {
            output::println_debug(
                (area.0.x..=area.1.x)
                    .map(|x| map_pos(IVec2::new(x, y)))
                    .collect(),
            );
        }
    }
}

pub fn print_grid<T>(grid: &[&[T]], map_to_char: fn(&T) -> char) {
    if output::is_debug_enabled() {
        for &row in grid {
            output::println_debug(row.iter().map(map_to_char).collect());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        let grid = ".#.\n..#\n###";
        let parsed = parse_set(grid.lines().map(|s| s.to_string()));

        assert_eq!(parsed.len(), 5);
        assert!(parsed.contains(&IVec2::new(1, 0)));
        assert!(parsed.contains(&IVec2::new(2, 1)));
        assert!(parsed.contains(&IVec2::new(0, 2)));
        assert!(parsed.contains(&IVec2::new(1, 2)));
        assert!(parsed.contains(&IVec2::new(2, 2)));
    }
}
