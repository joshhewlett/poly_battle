use crate::structs::*;
use crate::traits::*;
use sdl2::pixels::Color;
use std::collections::HashMap;

///
/// Player definition
///
static mut ID_COUNTER: u32 = 0;

pub struct Player {
    id: u32,
    game_object_type: GameObjectType,
    origin: Point,
    prev_origin: Option<Point>,
    sprite: Sprite,
    speed: u32,
    current_direction: Direction,
    coin_count: u32,
}

impl Player {
    pub fn new(origin: Point) -> Self {
        Player {
            id: Player::get_id(),
            game_object_type: GameObjectType::Player,
            origin,
            prev_origin: None,
            sprite: Sprite::new(Player::get_shape()),
            current_direction: Direction::default(),
            speed: 5,
            coin_count: 0,
        }
    }

    fn get_id() -> u32 {
        let id: u32;
        unsafe {
            id = ID_COUNTER;
            ID_COUNTER += 1;
        }

        id
    }

    fn get_shape() -> HashMap<Point, Pixel> {
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

        assert!(
            shape.len() < u32::MAX as usize,
            "Shape height larger than expected"
        );
        assert!(
            shape[0].len() < u32::MAX as usize,
            "Shape width larger than expected"
        );

        let mut pixels: HashMap<Point, Pixel> = HashMap::new();
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                if let Some(color) = shape[y][x] {
                    let location = Point::new(x as u32, y as u32);
                    pixels.insert(location.clone(), Pixel::new(&color));
                }
            }
        }

        pixels
    }

    pub fn increment_coin_count(&mut self) {
        self.coin_count += 1;
    }

    pub fn coin_count(&mut self) -> u32 {
        self.coin_count
    }

    pub fn change_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
}

impl GameObject for Player {
    fn game_object_type(&self) -> &GameObjectType {
        &self.game_object_type
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn origin(&self) -> &Point {
        &self.origin
    }

    fn set_origin(&mut self, new_origin: &Point) {
        self.origin = new_origin.clone();
    }

    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn sprite_dimensions(&self) -> &Dimensions {
        self.sprite.dimensions()
    }

    fn tick(&mut self) {
        self.apply_movement();
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point::default())
    }
}

impl Movable for Player {
    fn direction(&self) -> &Direction {
        &self.current_direction
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.current_direction = new_direction;
    }

    fn speed(&self) -> u32 {
        self.speed
    }

    fn prev_origin(&self) -> Option<&Point> {
        match &self.prev_origin {
            Some(point) => Some(point),
            None => None,
        }
    }

    fn set_prev_origin(&mut self, origin: &Point) {
        self.prev_origin = Some(origin.clone());
    }
}
