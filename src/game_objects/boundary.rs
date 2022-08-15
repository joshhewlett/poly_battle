use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::structs::{GameObjectType, Pixel, Shape};
use crate::traits::{Drawable, GameObject};

// TODO: Remove this
static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static CENTER_X: i32 = (WINDOW_WIDTH / 2) as i32;
static CENTER_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

pub struct Boundary {
    position: Point,
    shape: Shape,
}

impl Boundary {
    pub fn new() -> Self {
        Self {
            position: Point::new(0, 0),
            shape: Boundary::get_boundary_shape(),
        }
    }

    fn get_boundary_shape() -> Shape {
        let width = 5;
        let color = Color::WHITE;

        // Base 1d array
        let mut base_shape_data = vec![None; WINDOW_HEIGHT as usize * WINDOW_WIDTH as usize];

        // Vector of 'width' elements slices
        let mut grid_base: Vec<_> = base_shape_data
            .as_mut_slice()
            .chunks_mut(WINDOW_WIDTH as usize)
            .collect();

        // Final 2d array `&mut [&mut [_]]`
        let shape_data_grid = grid_base.as_mut_slice();

        // Fill top boundary
        for x in 0..(WINDOW_WIDTH as usize) {
            for y in 0..width {
                shape_data_grid[y][x] = Some(color.clone());
            }
        }

        let mut pixels: HashMap<Point, Pixel> = HashMap::new();
        for y in 0..shape_data_grid.len() {
            for x in 0..shape_data_grid[y].len() {
                let color = shape_data_grid[y][x];
                if color.is_some() {
                    let location = Point::new(x as i32, y as i32);
                    pixels.insert(location.clone(), Pixel::new(location, color.unwrap()));
                }
            }
        }

        Shape::new(pixels, shape_data_grid[0].len(), shape_data_grid.len())
    }
}

// Should Drawable be on GameObject?
impl Drawable for Boundary {
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

impl GameObject for Boundary {
    fn game_object_type(&self) -> GameObjectType {
        GameObjectType::Wall(0)
    }

    fn tick(&mut self) {
        ()
    }
}
