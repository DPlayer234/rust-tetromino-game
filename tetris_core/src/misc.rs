use std::ops::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

pub type Vec2I8 = Vec2<i8>;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T>  {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T>  {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(f: (T, T)) -> Self {
        Vec2::new(f.0, f.1)
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(f: Vec2<T>) -> Self {
        (f.x, f.y)
    }
}

impl<T: Default> Default for Vec2<T> {
    fn default() -> Self {
        Vec2::new(T::default(), T::default())
    }
}

impl<T: Eq> Eq for Vec2<T> {}
impl<T: Copy> Copy for Vec2<T> {}

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
