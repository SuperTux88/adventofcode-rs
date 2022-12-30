use glam::IVec2;

pub trait ManhattenDistance {
    fn manhattan_distance(&self, other: &Self) -> u32;
}

impl ManhattenDistance for IVec2 {
    fn manhattan_distance(&self, other: &IVec2) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}
