use glam::IVec2;

use super::directions::{Direction, DIRECTIONS};

pub trait Walk {
    fn move_step(&self, dir: Direction) -> IVec2;
    fn move_distance(&self, dir: Direction, distance: i32) -> IVec2;
    fn next_line(&self) -> IVec2;
}

impl Walk for IVec2 {
    fn move_step(&self, dir: Direction) -> IVec2 {
        *self + DIRECTIONS[dir as usize]
    }

    fn move_distance(&self, dir: Direction, distance: i32) -> IVec2 {
        *self + DIRECTIONS[dir as usize] * distance
    }

    fn next_line(&self) -> IVec2 {
        IVec2::new(0, self.y + 1)
    }
}
