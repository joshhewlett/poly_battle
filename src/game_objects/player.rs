use crate::structs::*;
use crate::traits::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashMap;

// TODO: Remove this
static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static CENTER_X: i32 = (WINDOW_WIDTH / 2) as i32;
static CENTER_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

///
/// Player definition
///
pub struct Player {
    position: Point,
    shape: Shape,
    current_direction: Direction,
    speed: i32,
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

    pub fn increment_coin_count(&mut self) {
        self.coin_count += 1;
    }

    pub fn coin_count(&mut self) -> i32 {
        self.coin_count
    }

    pub fn change_speed(&mut self, speed: i32) {
        self.speed = speed;
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point::new(CENTER_X, CENTER_Y))
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
    fn speed(&self) -> i32 {
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
