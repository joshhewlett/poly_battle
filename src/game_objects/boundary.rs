use std::collections::HashMap;
use sdl2::pixels::Color;
use crate::structs::*;
use crate::traits::{Drawable, GameObject};

static BOUNDARY_WIDTH: usize = 5;
static BOUNDARY_COLOR: Color = Color::WHITE;

pub struct Boundary {
    position: Point,
    shape: Shape,
}

impl Boundary {
    pub fn new(map_width: u32, map_height: u32) -> Self {
        Self {
            position: Point::new(0, 0),
            shape: Boundary::get_boundary_shape(map_width, map_height),
        }
    }

    fn get_boundary_shape(map_width: u32, map_height: u32) -> Shape {

        // Base 1d array
        let mut base_shape_data = vec![None; map_height as usize * map_width as usize];

        // Vector of 'width' elements slices
        let mut grid_base: Vec<_> = base_shape_data
            .as_mut_slice()
            .chunks_mut(map_width as usize)
            .collect();

        // Final 2d array `&mut [&mut [_]]`
        let shape_data_grid = grid_base.as_mut_slice();

        // Fill top and bottom boundary
        for x in 0..(map_width as usize) {
            for y in 0..BOUNDARY_WIDTH {
                // Top boundary
                shape_data_grid[y][x] = Some(BOUNDARY_COLOR.clone());

                // Bottom boundary
                shape_data_grid[(y + map_height as usize) - BOUNDARY_WIDTH][x] =
                    Some(BOUNDARY_COLOR.clone());
            }
        }

        // Fill left and right boundary
        for y in 0..(map_height as usize) {
            for x in 0..BOUNDARY_WIDTH {
                // Left boundary
                shape_data_grid[y][x] = Some(BOUNDARY_COLOR.clone());

                // Right boundary
                shape_data_grid[y][(x + map_width as usize) - 5] =
                    Some(BOUNDARY_COLOR.clone());
            }
        }

        // Convert 2D Vector to HashMap
        let mut pixels: HashMap<Point, Pixel> = HashMap::new();
        for y in 0..shape_data_grid.len() {
            for x in 0..shape_data_grid[y].len() {
                let color = shape_data_grid[y][x];
                if color.is_some() {
                    let location = Point::new(x as u32, y as u32);
                    pixels.insert(location.clone(), Pixel::new(location, color.unwrap()));
                }
            }
        }

        Shape::new(pixels, shape_data_grid[0].len(), shape_data_grid.len())
    }
}

// Should Drawable be on GameObject?
impl Drawable for Boundary {
    fn position(&self) -> Point {
        self.position.clone()
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

    }
}
