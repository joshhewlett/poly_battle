use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::structs::*;

///
/// GameObject definition
///
pub trait GameObject: Drawable {
    fn game_object_type(&self) -> GameObjectType;

    fn tick(&mut self) {
        // Do nothing by default
        ();
    }
}

///
/// Drawable definition
///
pub trait Drawable {
    fn position(&self) -> &Point;
    fn set_position(&mut self, point: Point);
    fn shape(&self) -> &Shape;

    // TODO: rename and implement memoization somehow
    // fn get_active_pixels(&self) -> Vec<(Point, &Option<Color>)> {
    fn get_active_pixels(&self) -> Vec<Pixel> {

        let position: &Point = &self.position();
        let x_origin = position.x;
        let y_origin = position.y;

        let shape: &Shape = &self.shape();

        let mut result: Vec<Pixel> = Vec::new();

        for y_ptr in 0..shape.height() {
            for x_ptr in 0..shape.width() {
                let pixel_color: &Option<Color> = shape.get_pixel(x_ptr, y_ptr);

                if pixel_color.is_some() {
                    let point = Point::new(
                        x_origin + i32::try_from(x_ptr).unwrap(),
                        y_origin + i32::try_from(y_ptr).unwrap(),
                    );
                    result.push(Pixel::new(point, pixel_color.unwrap()));
                }
            }
        }

        result
    }

    fn draw(&self, canvas: &mut WindowCanvas) {
        let pixels: Vec<Pixel> = self.get_active_pixels();

        pixels.iter().for_each(|pixel| {
            canvas.set_draw_color(pixel.color);
            canvas.draw_point(pixel.location).unwrap();
        });
    }
}

///
/// Collidable definition
///
pub trait Collidable: Drawable {
    fn has_collided(&self, other: &dyn Collidable) -> bool {
        let common_points: HashSet<Point> = self
            .get_active_pixels()
            .iter()
            .map(|pixel| pixel.location)
            .collect();

        other
            .get_active_pixels()
            .iter()
            .map(|p| p.location)
            .any(|p| common_points.contains(&p))
    }
}

///
/// Moveable definition
///
pub trait Moveable: Drawable {
    fn direction(&self) -> &Direction;
    fn change_direction(&mut self, new_direction: Direction);

    fn apply_movement(&mut self) {

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