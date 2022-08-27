use crate::structs::{Dimensions, GameObjectType, Pixel, Point, Sprite};
use crate::util::has_collided;
use std::collections::{HashMap, HashSet};

///
/// GameObject
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
