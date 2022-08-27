mod sprite;

pub use sprite::*;

use std::collections::HashMap;

///
/// Point definition
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Self { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0)
    }
}

///
/// Direction definition
///
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

///
/// Pixel definition
///
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub color: Color, // TODO: Replace with custom implementation
}

impl Pixel {
    pub fn new(color: Color) -> Self {
        Pixel {
            color: color.clone(),
        }
    }
}

///
/// Dimensions definition
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Dimensions { width, height }
    }
}

///
/// GameObjectType definition
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameObjectType {
    Player,
    Coin,
    Boundary,
}
