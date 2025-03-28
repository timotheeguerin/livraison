/// An amount of space in 2 dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    /// X pos
    pub x: i32,
    /// Y pos
    pub y: i32,
}

impl Position {
    /// Creates a new  [`Position`] with the given width and height.
    pub const fn new(width: i32, height: i32) -> Self {
        Position {
            x: width,
            y: height,
        }
    }
}

impl Position {
    /// A [`Position`] with zero width and height.
    pub const ZERO: Position = Position::new(0, 0);

    /// Expands this [`Position`] by the given amount.
    pub fn expand(self, other: impl Into<Position>) -> Self {
        let other = other.into();

        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<[i32; 2]> for Position {
    fn from([width, height]: [i32; 2]) -> Self {
        Position {
            x: width,
            y: height,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from((width, height): (i32, i32)) -> Self {
        Self {
            x: width,
            y: height,
        }
    }
}
impl From<Position> for [i32; 2] {
    fn from(position: Position) -> Self {
        [position.x, position.y]
    }
}

impl std::ops::Add for Position
where
    i32: std::ops::Add<Output = i32>,
{
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position
where
    i32: std::ops::Sub<Output = i32>,
{
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
