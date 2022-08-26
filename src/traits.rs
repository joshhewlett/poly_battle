use crate::game_util::has_collided;
use crate::structs::*;
use std::collections::{HashMap, HashSet};

///
/// GameObject definition
///
pub trait GameObject {
    fn tick(&mut self) {
        // Do nothing by default
    }

    fn game_object_type(&self) -> GameObjectType;
    fn id(&self) -> u32;
    fn origin(&self) -> Point;
    fn set_origin(&mut self, new_origin: Point);
    fn sprite(&self) -> &Sprite;
    fn sprite_dimensions(&self) -> Dimensions;
    fn effective_points(&self) -> &HashSet<Point>;
    fn effective_pixels(&self) -> &HashMap<Point, Pixel>;

    fn identity(&self) -> (GameObjectType, u32) {
        (self.game_object_type(), self.id())
    }

    fn has_collided_with(&self, other: Box<&dyn GameObject>) -> bool {
        has_collided(self.effective_points(), other.effective_points())
    }
}

///
/// Movable definition
///
pub trait Movable: GameObject {
    fn direction(&self) -> Direction;
    fn change_direction(&mut self, new_direction: Direction);
    fn speed(&self) -> u32;
    fn prev_origin(&self) -> Option<Point>;
    fn set_prev_origin(&mut self, current_origin: Point);

    fn apply_movement(&mut self) {
        let current_origin = self.origin().clone();
        let speed = self.speed();

        let new_origin: Point;
        match self.direction() {
            Direction::Up => {
                new_origin = Point::new(current_origin.x, current_origin.y - speed);
            }
            Direction::Down => {
                new_origin = Point::new(current_origin.x, current_origin.y + speed);
            }
            Direction::Left => {
                new_origin = Point::new(current_origin.x - speed, current_origin.y);
            }
            Direction::Right => {
                new_origin = Point::new(current_origin.x + speed, current_origin.y);
            }
        }

        self.set_prev_origin(current_origin);
        self.set_origin(new_origin);
    }

    fn prev_origin_unchecked(&self) -> Point {
        self.prev_origin().expect("Expected prev_origin to exist")
    }
}
