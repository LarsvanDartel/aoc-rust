use crate::common::Vec2;
use crate::ParseResult;
use nom::{branch::alt, bytes::complete::tag, Parser};

/// A cardinal direction.
/// North, East, South, West.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    /// All cardinal directions.
    pub fn cardinal() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .copied()
    }

    /// All ordinal directions.
    pub fn ordinal() -> impl Iterator<Item = Self> {
        [
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
        .iter()
        .copied()
    }

    /// All directions.
    pub fn all() -> impl Iterator<Item = Self> {
        Self::cardinal().chain(Self::ordinal())
    }

    /// Parse a cardinal direction from the input.
    /// Cardinal directions are represented by a single letter: N, E, S, W.
    /// N is North, E is East, S is South, W is West.
    pub fn parse_nsew(input: &str) -> ParseResult<Self> {
        alt((
            tag("N").map(|_| Direction::North),
            tag("E").map(|_| Direction::East),
            tag("S").map(|_| Direction::South),
            tag("W").map(|_| Direction::West),
        ))
        .parse(input)
    }

    /// Parse a direction from the input.
    /// Directions are represented by a single letter: U, R, D, L.
    /// U is Up, R is Right, D is Down, L is Left.
    pub fn parse_udlr(input: &str) -> ParseResult<Self> {
        alt((
            tag("U").map(|_| Direction::North),
            tag("R").map(|_| Direction::East),
            tag("D").map(|_| Direction::South),
            tag("L").map(|_| Direction::West),
        ))
        .parse(input)
    }

    /// Return the direction to the right of the current direction.
    /// North -> East, East -> South, South -> West, West -> North.
    pub const fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
        }
    }

    /// Return the direction to the left of the current direction.
    /// North -> West, West -> South, South -> East, East -> North.
    pub const fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::NorthEast => Direction::NorthWest,
            Direction::SouthEast => Direction::NorthEast,
            Direction::SouthWest => Direction::SouthEast,
            Direction::NorthWest => Direction::SouthWest,
        }
    }

    /// Return the direction opposite to the current direction.
    /// North -> South, South -> North, East -> West, West -> East.
    pub const fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthEast,
        }
    }
}

impl From<Direction> for Vec2<isize> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => Vec2::new(0, -1),
            Direction::East => Vec2::new(1, 0),
            Direction::South => Vec2::new(0, 1),
            Direction::West => Vec2::new(-1, 0),
            Direction::NorthEast => Vec2::new(1, -1),
            Direction::NorthWest => Vec2::new(-1, -1),
            Direction::SouthEast => Vec2::new(1, 1),
            Direction::SouthWest => Vec2::new(-1, 1),
        }
    }
}

impl std::ops::Add<Direction> for Vec2<isize> {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Vec2::from(rhs)
    }
}

impl std::ops::AddAssign<Direction> for Vec2<isize> {
    fn add_assign(&mut self, rhs: Direction) {
        *self += Vec2::from(rhs);
    }
}

impl std::ops::Sub<Direction> for Vec2<isize> {
    type Output = Self;

    fn sub(self, rhs: Direction) -> Self::Output {
        self - Vec2::from(rhs)
    }
}

impl std::ops::SubAssign<Direction> for Vec2<isize> {
    fn sub_assign(&mut self, rhs: Direction) {
        *self -= Vec2::from(rhs);
    }
}

impl std::ops::Mul<isize> for Direction {
    type Output = Vec2<isize>;

    fn mul(self, rhs: isize) -> Self::Output {
        Vec2::from(self) * rhs
    }
}
