use crate::traits::GameObject;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
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
#[derive(Debug, Clone)]
pub struct Pixel {
    pub location: Point,
    // TODO: Replace with custom implementation
    pub color: Color,
}

impl Pixel {
    pub fn new(location: Point, color: Color) -> Self {
        Self { location, color }
    }

    pub fn same_location(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            location: Point::new(0, 0),
            color: Color::WHITE,
        }
    }
}

///
/// Shape definition
///
pub struct Shape {
    // TODO: Replace with Pixel
    // shape: Vec<Vec<Option<Color>>>,
    // TODO: Consider renaming this
    shape: HashMap<Point, Pixel>,
    width: usize,
    height: usize,
}

// TODO: Should Shape implement Iterable?
impl Shape {
    pub fn new(shape: HashMap<Point, Pixel>, width: usize, height: usize) -> Self {
        Self {
            shape,
            width,
            height,
        }
    }

    pub fn shape_data(&self) -> &HashMap<Point, Pixel> {
        &self.shape
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.shape.get(&Point::new(x as i32, y as i32))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for Shape {
    fn default() -> Self {
        let row = vec![
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
        ];

        let mut shape: Vec<Vec<Option<Color>>> = Vec::new();
        for _ in 0..8 {
            shape.push(row.clone());
        }


        let mut pixels: HashMap<Point, Pixel> = HashMap::new();
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                let color = shape[y][x];
                if color.is_some() {
                    let location = Point::new(x as i32, y as i32);
                    pixels.insert(location.clone(), Pixel::new(location, color.unwrap()));
                }
            }
        }

        // Shape::new(shape)
        Shape::new(pixels, shape[0].len(), shape.len())
    }
}

pub struct CollisionRule {}

#[derive(Debug, Clone, PartialEq)]
pub enum GameObjectType {
    Player,
    Coin(i32),
    Wall(i32),
}
