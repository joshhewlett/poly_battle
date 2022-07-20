use std::collections::{HashMap, HashSet};
use sdl2::pixels::Color;
use sdl2::rect::Point;

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
#[derive(Debug, Clone, )]
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
    // pub fn new(shape: Vec<Vec<Option<Color>>>) -> Self {
    //
    //     // let shape_pixels = HashSet::new();
    //     //
    //     // for y in 0..shape.len() {
    //     //     for x in 0..shape[0].len() {
    //     //
    //     //         if shape[x][y] == Some(color) {
    //     //             let pixel = Pixel::new(Point::new(x, y), color);
    //     //         }
    //     //     }
    //     // }
    //
    //     Shape {
    //         shape,
    //         // shape_pixels: vec![None; shape.len()],
    //     }
    // }
    pub fn new(shape: HashMap<Point, Pixel>) -> Self {

        Self {
            shape,
            // shape_pixels: vec![None; shape.len()],
        }
    }

    // fn shape_data(&self) -> &Vec<Vec<Option<Color>>> {
    //     &self.shape
    // }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.shape.get(&Point::new(x as i32, y as i32))
    }

    pub fn width(&self) -> usize {
        self.shape[0].len()
    }

    pub fn height(&self) -> usize {
        self.shape.len()
    }
}

impl Default for Shape {
    fn default() -> Self {

        let shape: Vec<Vec<Option<Color>>> = vec![
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
        ];

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
        Shape::new(pixels);
    }
}

pub struct CollisionRule {}

pub enum GameObjectType {
    Player,
    Coin,
    Wall
}