use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
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
    fn position(&self) -> &Point;
    fn set_position(&mut self, point: Point);
    fn shape(&self) -> &Shape;
}

///
/// Collidable definition
///
pub trait Collidable: Drawable {
    fn has_collided(&self, other: &dyn Collidable) -> bool {
        // let common_points: HashSet<Point> = self
        //     .get_active_pixels()
        //     .iter()
        //     .map(|pixel| pixel.location)
        //     .collect();
        //
        // other
        //     .get_active_pixels()
        //     .iter()
        //     .map(|p| p.location)
        //     .any(|p| common_points.contains(&p))
        true
    }
}

///
/// Moveable definition
///
pub trait Moveable: Drawable {
    fn direction(&self) -> &Direction;
    fn change_direction(&mut self, new_direction: Direction);

    fn apply_movement(&mut self) {

        // println!("Player direction: {:#?}", self.direction());
        let new_position: Point;
        match &self.direction() {
            Direction::Up => {
                new_position = Point::new(self.position().x, self.position().y - 1);
            }
            Direction::Down => {
                new_position = Point::new(self.position().x, self.position().y + 1);
            }
            Direction::Left => {
                new_position = Point::new(self.position().x - 1, self.position().y);
            }
            Direction::Right => {
                new_position = Point::new(self.position().x + 1, self.position().y);
            }
        }

        self.set_position(new_position);
    }
}