/// An amount of space in 2 dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size {
    /// Width
    pub width: i32,
    /// Height
    pub height: i32,
}

impl Size {
    /// Creates a new  [`Size`] with the given width and height.
    pub const fn new(width: i32, height: i32) -> Self {
        Size { width, height }
    }
}

impl Size {
    /// A [`Size`] with zero width and height.
    pub const ZERO: Size = Size::new(0, 0);

    /// Expands this [`Size`] by the given amount.
    pub fn expand(self, other: impl Into<Size>) -> Self {
        let other = other.into();

        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl From<[i32; 2]> for Size {
    fn from([width, height]: [i32; 2]) -> Self {
        Size { width, height }
    }
}

impl From<(i32, i32)> for Size {
    fn from((width, height): (i32, i32)) -> Self {
        Self { width, height }
    }
}
impl From<Size> for [i32; 2] {
    fn from(size: Size) -> Self {
        [size.width, size.height]
    }
}

impl std::ops::Add for Size
where
    i32: std::ops::Add<Output = i32>,
{
    type Output = Size;

    fn add(self, rhs: Self) -> Self::Output {
        Size {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl std::ops::Sub for Size
where
    i32: std::ops::Sub<Output = i32>,
{
    type Output = Size;

    fn sub(self, rhs: Self) -> Self::Output {
        Size {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}
