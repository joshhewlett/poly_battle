use crate::structs::*;
use crate::traits::*;

///
/// Coin definition
///
static mut ID_COUNTER: u32 = 0;

pub struct Coin {
    id: u32,
    game_object_type: GameObjectType,
    origin: Point,
    sprite: Sprite,
}

impl Coin {
    pub fn new(origin: Point) -> Self {
        Coin {
            id: Coin::get_id(),
            game_object_type: GameObjectType::Coin,
            origin,
            sprite: Sprite::default(),
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
    fn id(&self) -> u32 {
        self.id
    }

    fn game_object_type(&self) -> &GameObjectType {
        &self.game_object_type
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
}

impl Default for Coin {
    fn default() -> Self {
        Coin::new(Point::default())
    }
}
