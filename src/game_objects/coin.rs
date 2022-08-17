use crate::structs::*;
use crate::traits::*;

///
/// Coin definition
///
pub struct Coin {
    position: Point,
    shape: Shape,
    id: i32,
}

impl Coin {
    pub fn new(position: Point) -> Self {
        Coin {
            position,
            shape: Shape::default(),
            id: 0,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl Default for Coin {
    fn default() -> Self {
        Coin::new(Point::default())
    }
}

// TODO: This is probably a good use of #derive!
impl Drawable for Coin {
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

impl Collidable for Coin {}

impl GameObject for Coin {
    fn game_object_type(&self) -> GameObjectType {
        GameObjectType::Coin(self.id())
    }
}
