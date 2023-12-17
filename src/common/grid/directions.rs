use glam::IVec2;

pub const UP: IVec2 = IVec2::NEG_Y;
pub const RIGHT: IVec2 = IVec2::X;
pub const DOWN: IVec2 = IVec2::Y;
pub const LEFT: IVec2 = IVec2::NEG_X;

pub const TOP_LEFT: IVec2 = IVec2::new(-1, -1);
pub const TOP_RIGHT: IVec2 = IVec2::new(1, -1);
pub const BOTTOM_RIGHT: IVec2 = IVec2::new(1, 1);
pub const BOTTOM_LEFT: IVec2 = IVec2::new(-1, 1);

pub const DIRECTIONS: [IVec2; 4] = [UP, RIGHT, DOWN, LEFT];
pub const DIAGONALS: [IVec2; 4] = [TOP_LEFT, TOP_RIGHT, BOTTOM_RIGHT, BOTTOM_LEFT];
pub const NEIGHBORS: [IVec2; 8] = [
    TOP_LEFT,
    UP,
    TOP_RIGHT,
    RIGHT,
    BOTTOM_RIGHT,
    DOWN,
    BOTTOM_LEFT,
    LEFT,
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' | 'N' | 'U' => Direction::Up,
            '>' | 'E' | 'R' => Direction::Right,
            'v' | 'S' | 'D' => Direction::Down,
            '<' | 'W' | 'L' => Direction::Left,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Debug)]
pub enum Turn {
    Left,
    Right,
    Flip,
}

impl Direction {
    pub fn turn(&self, turn: &Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            Turn::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Turn::Flip => match self {
                Direction::Up => Direction::Down,
                Direction::Right => Direction::Left,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
            },
        }
    }
}

pub trait Directions {
    fn up(&self) -> Self;
    fn right(&self) -> Self;
    fn down(&self) -> Self;
    fn left(&self) -> Self;

    fn directions(&self) -> Vec<Self>
    where
        Self: Sized;

    fn neighbors(&self) -> Vec<Self>
    where
        Self: Sized;

    fn direction(&self, other: &Self) -> Self
    where
        Self: Sized;

    fn line_to(&self, other: &Self) -> Box<dyn Iterator<Item = Self> + '_>
    where
        Self: Sized;
}

impl Directions for IVec2 {
    fn up(&self) -> Self {
        *self + UP
    }

    fn right(&self) -> Self {
        *self + RIGHT
    }

    fn down(&self) -> Self {
        *self + DOWN
    }

    fn left(&self) -> Self {
        *self + LEFT
    }
    fn directions(&self) -> Vec<Self> {
        DIRECTIONS.into_iter().map(|dir| *self + dir).collect()
    }

    fn neighbors(&self) -> Vec<Self> {
        NEIGHBORS.into_iter().map(|dir| *self + dir).collect()
    }

    fn direction(&self, other: &Self) -> Self {
        let x = other.x.cmp(&self.x);
        let y = other.y.cmp(&self.y);
        IVec2::new(x as i32, y as i32)
    }

    /// Returns an iterator over the line between `self` and `other`.
    /// The iterator includes `self` and `other`.
    ///
    /// ```
    /// # use glam::IVec2;
    /// # use adventofcode::common::grid::directions::Directions;
    /// #
    /// let start = IVec2::new(2, 0);
    /// let end = IVec2::new(5, 3);
    /// let line = start.line_to(&end).collect::<Vec<_>>();
    /// assert_eq!(line, vec![IVec2::new(2, 0), IVec2::new(3, 1), IVec2::new(4, 2), IVec2::new(5, 3)]);
    /// ```
    fn line_to(&self, other: &Self) -> Box<dyn Iterator<Item = Self> + '_> {
        let direction = self.direction(other);
        let diff = *other - *self;
        let distance = diff.x.max(diff.y).abs();
        Box::new((0..=distance).map(move |i| *self + direction * i))
    }
}
