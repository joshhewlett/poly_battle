use crate::structs::*;
use crate::traits::*;
use sdl2::pixels::Color;
use std::collections::HashMap;

///
/// Player definition
///
pub struct Player {
    position: Point,
    shape: Shape,
    current_direction: Direction,
    speed: u32,
    coin_count: i32,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Player {
            position,
            shape: Player::get_shape(),
            current_direction: Direction::default(),
            speed: 5,
            coin_count: 0,
        }
    }

    fn get_shape() -> Shape {
        let row = vec![
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::RED),
            Some(Color::RED),
        ];

        let mut shape: Vec<Vec<Option<Color>>> = Vec::new();
        for _ in 0..8 {
            shape.push(row.clone());
        }

        assert!(shape.len() < u32::MAX as usize, "Shape height larger than expected");
        assert!(shape[0].len() < u32::MAX as usize, "Shape width larger than expected");

        let mut pixels: HashMap<Point, Pixel> = HashMap::new();
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {

                if let Some(color) = shape[y][x] {
                    let location = Point::new(x as u32, y as u32);
                    pixels.insert(location.clone(), Pixel::new(location, color));
                }
            }
        }

        // Shape::new(shape)
        Shape::new(pixels, shape[0].len(), shape.len())
    }

    pub fn increment_coin_count(&mut self) {
        self.coin_count += 1;
    }

    pub fn coin_count(&mut self) -> i32 {
        self.coin_count
    }

    pub fn change_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point::default())
    }
}

// Should Drawable be on GameObject?
impl Drawable for Player {
    fn position(&self) -> Point {
        self.position.clone()
    }

    fn set_position(&mut self, new_position: Point) {
        self.position = new_position;
    }

    fn shape(&self) -> &Shape {
        &self.shape
    }
}

impl Collidable for Player {}

impl Moveable for Player {
    fn direction(&self) -> &Direction {
        &self.current_direction
    }
    fn speed(&self) -> u32 {
        self.speed
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.current_direction = new_direction;
    }
}

impl GameObject for Player {
    fn game_object_type(&self) -> GameObjectType {
        GameObjectType::Player
    }

    fn tick(&mut self) {
        self.apply_movement();
    }
}
