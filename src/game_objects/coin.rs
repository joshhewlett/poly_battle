use crate::structs::*;
use crate::traits::*;
use sdl2::rect::Point;

// TODO: Remove this
static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static CENTER_X: i32 = (WINDOW_WIDTH / 2) as i32;
static CENTER_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

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
        Coin::new(Point::new(CENTER_X + 50, CENTER_Y + 50))
    }
}

// TODO: This is probably a good use of #derive!
impl Drawable for Coin {
    fn position(&self) -> &Point {
        &self.position
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
