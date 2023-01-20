use std::ops::*;

pub mod pieces;
pub mod game;

#[derive(Clone, Debug, PartialEq)]
pub struct VecInt<T> {
    pub x: T,
    pub y: T
}

pub type Vec2U8 = VecInt<u8>;
pub type Vec2I8 = VecInt<i8>;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl<T> VecInt<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for VecInt<T>  {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign> AddAssign for VecInt<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for VecInt<T>  {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign> SubAssign for VecInt<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Neg<Output = T>> Neg for VecInt<T> {
    type Output = Self;

    fn neg(self) -> Self {
        VecInt::new(-self.x, -self.y)
    }
}

impl<T> From<(T, T)> for VecInt<T> {
    fn from(f: (T, T)) -> Self {
        VecInt::new(f.0, f.1)
    }
}

impl<T: Default> Default for VecInt<T> {
    fn default() -> Self {
        VecInt::new(T::default(), T::default())
    }
}

impl<T: Eq> Eq for VecInt<T> {}
impl<T: Copy> Copy for VecInt<T> {}

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
