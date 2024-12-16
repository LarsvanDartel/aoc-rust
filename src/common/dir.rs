use winnow::combinator::alt;
use winnow::prelude::*;

use crate::common::Vec2;

/// A cardinal direction.
/// North, East, South, West.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub fn parse_nsew(input: &mut &str) -> PResult<Self> {
        alt((
            'N'.map(|_| Direction::North),
            'E'.map(|_| Direction::East),
            'S'.map(|_| Direction::South),
            'W'.map(|_| Direction::West),
        ))
        .parse_next(input)
    }

    /// Parse a direction from the input.
    /// Directions are represented by a single letter: U, R, D, L.
    /// U is Up, R is Right, D is Down, L is Left.
    pub fn parse_udlr(input: &mut &str) -> PResult<Self> {
        alt((
            'U'.map(|_| Direction::North),
            'R'.map(|_| Direction::East),
            'D'.map(|_| Direction::South),
            'L'.map(|_| Direction::West),
        ))
        .parse_next(input)
    }

    /// Parse a direction from the input.
    /// Directions are represented by a single character: ^, >, v, <.
    /// ^ is North, > is East, v is South, < is West.
    pub fn parse_arrows(input: &mut &str) -> PResult<Self> {
        alt((
            '^'.map(|_| Direction::North),
            '>'.map(|_| Direction::East),
            'v'.map(|_| Direction::South),
            '<'.map(|_| Direction::West),
        ))
        .parse_next(input)
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

    /// Return whether the direction is cardinal.
    pub const fn is_cardinal(self) -> bool {
        matches!(
            self,
            Direction::North | Direction::East | Direction::South | Direction::West
        )
    }

    /// Return whether the direction is ordinal.
    pub const fn is_ordinal(self) -> bool {
        matches!(
            self,
            Direction::NorthEast
                | Direction::SouthEast
                | Direction::SouthWest
                | Direction::NorthWest
        )
    }

    /// Return whether the direction is vertical.
    pub const fn is_vertical(self) -> bool {
        matches!(self, Direction::North | Direction::South)
    }

    /// Return whether the direction is horizontal.
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Direction::East | Direction::West)
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
