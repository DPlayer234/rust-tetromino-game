use std::ops::*;

pub mod pieces;
pub mod game;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2Int<T> {
    pub x: T,
    pub y: T
}

pub type Vec2U8 = Vec2Int<u8>;
pub type Vec2I8 = Vec2Int<i8>;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl<T> Vec2Int<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vec2Int<T>  {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> AddAssign for Vec2Int<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2Int<T>  {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign> SubAssign for Vec2Int<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2Int<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2Int::new(-self.x, -self.y)
    }
}

impl<T> From<(T, T)> for Vec2Int<T> {
    fn from(f: (T, T)) -> Self {
        Vec2Int::new(f.0, f.1)
    }
}

impl<T> From<Vec2Int<T>> for (T, T) {
    fn from(f: Vec2Int<T>) -> Self {
        (f.x, f.y)
    }
}

impl<T: Default> Default for Vec2Int<T> {
    fn default() -> Self {
        Vec2Int::new(T::default(), T::default())
    }
}

impl<T: Eq> Eq for Vec2Int<T> {}
impl<T: Copy> Copy for Vec2Int<T> {}

impl Color {
    pub const fn black() -> Self { Color::new(0x00, 0x00, 0x00) }
    pub const fn white() -> Self { Color::new(0xff, 0xff, 0xff) }

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
}

impl Default for Color {
    fn default() -> Self { Color::black() }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}
