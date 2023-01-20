//! Holds misc. types used throughout this library.

use std::ops::*;

/// Defines a 2D Vector used to represent points and directions.
/// This supplies blanket implementations based on its parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

/// Defines a 2D Vector of [`i8`] used to represent points and directions.
/// Shorthand for [`Vec2`]`<`[`i8`]`>`.
pub type Vec2I8 = Vec2<i8>;

/// Defines a 3-component, 24-bit RGB color.
#[derive(Copy, Clone, Debug, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl<T> Vec2<T> {
    /// Creates a new vector with x and y components.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Rhs, Output = Out>, Rhs, Out> Add<Vec2<Rhs>> for Vec2<T>  {
    type Output = Vec2<Out>;

    fn add(self, rhs: Vec2<Rhs>) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign<Rhs>, Rhs> AddAssign<Vec2<Rhs>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<Rhs>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Rhs, Output = Out>, Rhs, Out> Sub<Vec2<Rhs>> for Vec2<T>  {
    type Output = Vec2<Out>;

    fn sub(self, rhs: Vec2<Rhs>) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign<Rhs>, Rhs> SubAssign<Vec2<Rhs>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<Rhs>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Neg<Output = Out>, Out> Neg for Vec2<T> {
    type Output = Vec2<Out>;

    fn neg(self) -> Self::Output {
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
    /// The pure black color (Hex: 000000)
    pub const BLACK: Color = Color::new(0x00, 0x00, 0x00);

    /// The pure white color (Hex: FFFFFF)
    pub const WHITE: Color = Color::new(0xff, 0xff, 0xff);

    /// Creates a new color from its 8-bit R, G, and B components.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Determines if the present color is pure black.
    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
}

impl Default for Color {
    fn default() -> Self { Color::BLACK }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}
