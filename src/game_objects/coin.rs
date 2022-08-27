use std::collections::{HashMap, HashSet};

use crate::structs::*;
use crate::traits::*;
use crate::util::calc_effective_sprite_pixels;

///
/// Coin definition
///
static mut ID_COUNTER: u32 = 0;

pub struct Coin {
    id: u32,
    game_object_type: GameObjectType,
    origin: Point,
    sprite: Sprite,
    effective_sprite_pixels: HashMap<Point, Pixel>,
    effective_sprite_points: HashSet<Point>,
}

impl Coin {
    pub fn new(origin: Point) -> Self {
        let sprite = Sprite::default();
        let (effective_sprite_pixels, effective_sprite_points) =
            calc_effective_sprite_pixels(&sprite, origin);

        Self {
            id: Coin::get_id(),
            game_object_type: GameObjectType::Coin,
            origin,
            sprite: Sprite::default(),
            effective_sprite_pixels,
            effective_sprite_points,
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
}

impl GameObject for Coin {
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
        self.origin = new_origin;
    }

    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn sprite_dimensions(&self) -> Dimensions {
        *self.sprite.dimensions()
    }

    fn effective_points(&self) -> &HashSet<Point> {
        &self.effective_sprite_points
    }

    fn effective_pixels(&self) -> &HashMap<Point, Pixel> {
        &self.effective_sprite_pixels
    }
}

impl Default for Coin {
    fn default() -> Self {
        Coin::new(Point::default())
    }
}
