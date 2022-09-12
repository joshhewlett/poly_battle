use serde::Deserialize;

///
/// Structs
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

///
/// Point implementation
///
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Self { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0)
    }
}

///
/// Pixel implementation
///
impl Pixel {
    pub fn new(color: Color) -> Self {
        Pixel {
            color: color.clone(),
        }
    }
}

///
/// Color implementation
///
impl Color {
    pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const WHITE: Color = Color::RGB(255, 255, 255);
    pub const BLACK: Color = Color::RGB(0, 0, 0);
    pub const RED: Color = Color::RGB(255, 0, 0);
    pub const GREEN: Color = Color::RGB(0, 255, 0);
    pub const BLUE: Color = Color::RGB(0, 0, 255);
}
