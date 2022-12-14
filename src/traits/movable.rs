use crate::structs::{Direction, Point, Rotation};
use crate::traits::GameObject;

///
/// Movable
///
pub trait Movable: GameObject {
    fn direction(&self) -> Direction;
    fn change_direction(&mut self, new_direction: Direction);
    fn rotation(&self) -> Rotation;
    fn rotation_enabled(&self) -> bool;
    fn disable_rotation(&mut self);
    fn enable_rotation(&mut self);
    fn change_rotation(&mut self, new_rotation: Rotation);
    fn speed(&self) -> u32;
    fn prev_origin(&self) -> Option<Point>;
    fn set_prev_origin(&mut self, current_origin: Point);

    fn apply_movement(&mut self) {
        let current_origin = self.origin();
        let speed = self.speed() as i32;

        let new_origin: Point;
        match self.direction() {
            Direction::Up => {
                new_origin = Point::new(current_origin.x, current_origin.y - speed);
                if self.rotation_enabled() {
                    self.change_rotation(Rotation::Up);
                }
            }
            Direction::Down => {
                new_origin = Point::new(current_origin.x, current_origin.y + speed);
                if self.rotation_enabled() {
                    self.change_rotation(Rotation::Down);
                }
            }
            Direction::Left => {
                new_origin = Point::new(current_origin.x - speed, current_origin.y);
                if self.rotation_enabled() {
                    self.change_rotation(Rotation::Left);
                }
            }
            Direction::Right => {
                new_origin = Point::new(current_origin.x + speed, current_origin.y);
                if self.rotation_enabled() {
                    self.change_rotation(Rotation::Right);
                }
            }
            Direction::Stopped => new_origin = current_origin,
        }

        self.set_origin(new_origin);
    }

    fn prev_origin_unchecked(&self) -> Point {
        self.prev_origin().expect("Expected prev_origin to exist")
    }
}
