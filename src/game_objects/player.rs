use sdl2::rect::Point;
use crate::traits::*;
use crate::structs::*;

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
}

impl Player {
    pub fn new(position: Point) -> Self {
        Player {
            position,
            shape: Shape::default(),
            current_direction: Direction::default(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point::new(CENTER_X, CENTER_Y))
    }
}

impl Drawable for Player {
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

impl Collidable for Player {}

impl Moveable for Player {
    fn direction(&self) -> &Direction {
        &self.current_direction
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
