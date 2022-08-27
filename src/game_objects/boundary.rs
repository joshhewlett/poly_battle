use crate::game_util::calc_effective_sprite_pixels;
use crate::structs::*;
use crate::traits::GameObject;
use sdl2::keyboard::Keycode::Hash;
use std::collections::{HashMap, HashSet};

static BOUNDARY_WIDTH: usize = 5;
static BOUNDARY_COLOR: Color = Color::WHITE;
static mut ID_COUNTER: u32 = 0;

pub struct Boundary {
    id: u32,
    game_object_type: GameObjectType,
    origin: Point,
    sprite: Sprite,
    effective_sprite_pixels: HashMap<Point, Pixel>,
    effective_sprite_points: HashSet<Point>,
}

impl Boundary {
    pub fn new(map_width: u32, map_height: u32) -> Self {
        let origin = Point::new(0, 0);
        let sprite = Sprite::new(Boundary::get_boundary_shape(map_width, map_height));
        let (effective_sprite_pixels, effective_sprite_points) =
            calc_effective_sprite_pixels(&sprite, origin);

        Boundary {
            id: Boundary::get_id(),
            game_object_type: GameObjectType::Boundary,
            origin,
            sprite,
            effective_sprite_pixels,
            effective_sprite_points,
        }
    }

    fn get_id() -> u32 {
        let id: u32;
        unsafe {
            id = ID_COUNTER;
            ID_COUNTER += 1;
        }

        id
    }

    fn get_boundary_shape(map_width: u32, map_height: u32) -> HashMap<Point, Pixel> {
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
                shape_data_grid[y][(x + map_width as usize) - 5] = Some(BOUNDARY_COLOR.clone());
            }
        }

        // Convert 2D Vector to HashMap
        let mut pixels: HashMap<Point, Pixel> = HashMap::new();
        for y in 0..shape_data_grid.len() {
            for x in 0..shape_data_grid[y].len() {
                let color = shape_data_grid[y][x];
                if color.is_some() {
                    let location = Point::new(x as u32, y as u32);
                    pixels.insert(location.clone(), Pixel::new(color.unwrap()));
                }
            }
        }

        pixels
    }
}

impl GameObject for Boundary {
    fn game_object_type(&self) -> GameObjectType {
        self.game_object_type
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn origin(&self) -> Point {
        self.origin
    }

    fn set_origin(&mut self, new_origin: Point) {
        self.origin = new_origin;
    }

    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn sprite_dimensions(&self) -> Dimensions {
        *self.sprite.dimensions()
    }

    fn effective_pixels(&self) -> &HashMap<Point, Pixel> {
        &self.effective_sprite_pixels
    }

    fn effective_points(&self) -> &HashSet<Point> {
        &self.effective_sprite_points
    }
}
