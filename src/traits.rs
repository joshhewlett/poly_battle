use crate::structs::*;

///
/// GameObject definition
///
pub trait GameObject {
    fn game_object_type(&self) -> &GameObjectType;
    fn id(&self) -> u32;
    fn origin(&self) -> &Point;
    fn set_origin(&mut self, new_origin: &Point);
    fn sprite(&self) -> &Sprite;
    fn sprite_dimensions(&self) -> &Dimensions;

    fn expanded_id(&self) -> String {
        format!("{:?}:{}", self.game_object_type(), self.id())
    }

    fn tick(&mut self) {
        // Do nothing by default
    }

    fn has_collided_with(&mut self, other: &dyn GameObject) -> bool {
        // TODO
        false
    }
}

///
/// Moveable definition
///
pub trait Moveable: GameObject {
    fn direction(&self) -> &Direction;
    fn change_direction(&mut self, new_direction: Direction);
    fn speed(&self) -> u32;

    fn apply_movement(&mut self) {
        let new_origin: Point;
        match &self.direction() {
            Direction::Up => {
                new_origin = Point::new(self.origin().x, self.origin().y - self.speed());
            }
            Direction::Down => {
                new_origin = Point::new(self.origin().x, self.origin().y + self.speed());
            }
            Direction::Left => {
                new_origin = Point::new(self.origin().x - self.speed(), self.origin().y);
            }
            Direction::Right => {
                new_origin = Point::new(self.origin().x + self.speed(), self.origin().y);
            }
        }

        self.set_origin(&new_origin);
    }
}
