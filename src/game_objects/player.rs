use std::collections::{HashMap, HashSet};

use crate::structs::*;
use crate::traits::*;
use crate::util::calc_effective_sprite_pixels;

///
/// Player definition
///
pub const PLAYER_SPRITE_FILENAME: &'static str = "player_sprite_2";
static mut ID_COUNTER: u32 = 0;

pub struct Player {
    id: u32,
    game_object_type: GameObjectType,
    origin: Point,
    prev_origin: Option<Point>,
    sprite: Sprite,
    effective_sprite_pixels: HashMap<Point, Pixel>,
    effective_sprite_points: HashSet<Point>,
    speed: u32,
    current_direction: Direction,
    current_rotation: Rotation,
    coin_count: u32,
}

impl Player {
    pub fn new(origin: Point) -> Self {
        let sprite = Sprite::new_from_file(PLAYER_SPRITE_FILENAME);
        let (effective_sprite_pixels, effective_sprite_points) =
            calc_effective_sprite_pixels(&sprite, origin);
        Player {
            id: Player::get_id(),
            game_object_type: GameObjectType::Player,
            origin,
            prev_origin: None,
            sprite,
            effective_sprite_pixels,
            effective_sprite_points,
            current_direction: Direction::Up,
            current_rotation: Rotation::Up,
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
    fn tick(&mut self) {
        let prev_rotation = self.current_rotation;

        self.apply_movement();

        let mut something_changed = false;
        if self.origin != self.prev_origin_unchecked() {
            something_changed = true;
        }
        if self.current_rotation != prev_rotation {
            something_changed = true;
            self.sprite
                .rotate_sprite_around_origin(self.current_rotation);
        }

        if something_changed {
            let (effective_sprite_pixels, effective_sprite_points) =
                calc_effective_sprite_pixels(&self.sprite(), self.origin);
            self.effective_sprite_pixels = effective_sprite_pixels;
            self.effective_sprite_points = effective_sprite_points;
        }
    }

    fn game_object_type(&self) -> GameObjectType {
        self.game_object_type
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn origin(&self) -> Point {
        self.origin
    }

    fn set_origin(&mut self, new_origin: Point) {
        self.set_prev_origin(self.origin);
        self.origin = new_origin;
    }

    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn sprite_dimensions(&self) -> Dimensions {
        *self.sprite.dimensions()
    }

    fn effective_pixels(&self) -> &HashMap<Point, Pixel> {
        &self.effective_sprite_pixels
    }

    fn effective_points(&self) -> &HashSet<Point> {
        &self.effective_sprite_points
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point::default())
    }
}

impl Movable for Player {
    fn direction(&self) -> Direction {
        self.current_direction
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.current_direction = new_direction;
    }

    fn rotation(&self) -> Rotation {
        self.current_rotation
    }

    fn change_rotation(&mut self, new_rotation: Rotation) {
        self.current_rotation = new_rotation;
    }

    fn speed(&self) -> u32 {
        self.speed
    }

    fn prev_origin(&self) -> Option<Point> {
        match self.prev_origin {
            Some(point) => Some(point),
            None => None,
        }
    }

    fn set_prev_origin(&mut self, origin: Point) {
        self.prev_origin = Some(origin);
    }
}
