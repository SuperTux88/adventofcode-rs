use glam::IVec2;

pub trait ManhattenDistance {
    fn manhattan_distance(&self, other: Self) -> i32;
}

impl ManhattenDistance for IVec2 {
    fn manhattan_distance(&self, other: IVec2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
