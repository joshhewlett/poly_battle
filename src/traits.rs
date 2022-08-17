use crate::structs::*;

///
/// GameObject definition
///
pub trait GameObject {
    fn game_object_type(&self) -> GameObjectType;

    fn tick(&mut self) {
        // Do nothing by default
        ();
    }
}

///
/// Drawable definition
///
pub trait Drawable: GameObject {
    fn position(&self) -> Point;
    fn set_position(&mut self, point: Point);
    fn shape(&self) -> &Shape;
}

///
/// Collidable definition
///
pub trait Collidable: Drawable {

    // TODO: Is this needed?
    fn has_collided(&self, _other: &dyn Collidable) -> bool {

        true
    }
}

///
/// Moveable definition
///
pub trait Moveable: Drawable {
    fn direction(&self) -> &Direction;
    fn change_direction(&mut self, new_direction: Direction);
    fn speed(&self) -> u32;

    fn apply_movement(&mut self) {
        // println!("Player direction: {:#?}", self.direction());
        let new_position: Point;
        match &self.direction() {
            Direction::Up => {
                new_position = Point::new(self.position().x, self.position().y - self.speed());
            }
            Direction::Down => {
                new_position = Point::new(self.position().x, self.position().y + self.speed());
            }
            Direction::Left => {
                new_position = Point::new(self.position().x - self.speed(), self.position().y);
            }
            Direction::Right => {
                new_position = Point::new(self.position().x + self.speed(), self.position().y);
            }
        }

        self.set_position(new_position);
    }
}
