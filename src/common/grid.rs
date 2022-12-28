use std::collections::HashSet;

use glam::IVec2;

pub mod directions;
pub mod distance;
pub mod walk;

pub fn parse_set(lines: impl Iterator<Item = String>) -> HashSet<IVec2> {
    lines
        .enumerate()
        .flat_map(|(y, line)| {
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
