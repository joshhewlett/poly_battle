use sdl2::pixels::Color;
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
    pub fn new(color: &Color) -> Self {
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
/// Sprite definition
///
#[derive(Debug)]
pub struct Sprite {
    dimensions: Dimensions,
    sprite_data: HashMap<Point, Pixel>,
}

impl Sprite {
    pub fn new(sprite_data: HashMap<Point, Pixel>) -> Self {
        let mut x_max: u32 = 0;
        let mut y_max: u32 = 0;

        // Find width and height of sprite
        for (point, _pixel) in sprite_data.iter() {
            if point.x >= x_max {
                x_max = point.x
            }
            if point.y >= y_max {
                y_max = point.y
            }
        }

        Sprite {
            dimensions: Dimensions::new(x_max, y_max),
            sprite_data,
        }
    }

    // TODO
    // pub fn new_from_file(file_name: &str) -> Self {
    // }

    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }

    pub fn pixels(&self) -> &HashMap<Point, Pixel> {
        &self.sprite_data
    }

    fn build_default_sprite() -> HashMap<Point, Pixel> {
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

        assert!(
            shape.len() < u32::MAX as usize,
            "Shape height larger than expected"
        );
        assert!(
            shape[0].len() < u32::MAX as usize,
            "Shape width larger than expected"
        );
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                let color = shape[y][x];
                if color.is_some() {
                    let location = Point::new(x as u32, y as u32);
                    pixels.insert(location.clone(), Pixel::new(&color.unwrap()));
                }
            }
        }

        pixels
    }
}

// TODO delete this
impl Default for Sprite {
    fn default() -> Self {
        Sprite::new(Sprite::build_default_sprite())
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
