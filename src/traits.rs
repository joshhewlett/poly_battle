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
/// Movable definition
///
pub trait Movable: GameObject {
    fn direction(&self) -> &Direction;
    fn change_direction(&mut self, new_direction: Direction);
    fn speed(&self) -> u32;
    fn prev_origin(&self) -> Option<&Point>;
    fn set_prev_origin(&mut self, current_origin: &Point);

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

        self.set_prev_origin(&current_origin);
        self.set_origin(&new_origin);
    }
}
