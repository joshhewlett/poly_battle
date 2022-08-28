use crate::structs::{Color, Pixel, Point, Rotation};
use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel as ImagePixel};
use std::collections::HashMap;

pub const SPRITE_RESOURCE_DIR: &'static str = "resources/sprites/";

///
/// Structs
///
#[derive(Debug)]
pub struct Sprite {
    dimensions: Dimensions,
    original_sprite_data: HashMap<Point, Pixel>,
    sprite_data: HashMap<Point, Pixel>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

///
/// Sprite implementation
///
impl Sprite {
    pub fn new(sprite_data: HashMap<Point, Pixel>) -> Self {
        let mut x_max: u32 = 0;
        let mut y_max: u32 = 0;

        // Find width and height of sprite
        for (point, _pixel) in sprite_data.iter() {
            if point.x >= x_max {
                x_max = point.x
            }
            if point.y >= y_max {
                y_max = point.y
            }
        }

        Sprite {
            dimensions: Dimensions::new(x_max, y_max),
            original_sprite_data: sprite_data.clone(),
            sprite_data,
        }
    }

    pub fn new_from_file(file_name: &str) -> Self {
        let img = ImageReader::open(SPRITE_RESOURCE_DIR.to_owned() + file_name)
            .unwrap()
            .decode()
            .unwrap();

        let img_width = img.width();
        let img_height = img.height();

        let mut sprite_data: HashMap<Point, Pixel> = HashMap::new();
        img.pixels()
            .filter(|(_x, _y, (rgba))| rgba[0] != 0 || rgba[1] != 0 || rgba[2] != 0 || rgba[3] != 0)
            .for_each(|(x, y, (rgba))| {
                sprite_data.insert(
                    Point::new(x, y),
                    Pixel::new(Color::RGBA(rgba[0], rgba[1], rgba[2], rgba[3])),
                );
            });

        Self {
            dimensions: Dimensions::new(img_width, img_height),
            original_sprite_data: sprite_data.clone(),
            sprite_data,
        }
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }

    pub fn pixels(&self) -> &HashMap<Point, Pixel> {
        &self.sprite_data
    }

    pub fn rotate_sprite(&mut self, rotation: Rotation) {
        let center_width: i64 = (self.dimensions.width / 2) as i64;
        let center_height: i64 = (self.dimensions.height / 2) as i64;

        match rotation {
            Rotation::None => self.sprite_data = self.original_sprite_data.clone(),
            Rotation::Left => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .for_each(|(Point { x, y }, pixel)| {
                        let current_x: i64 = if (*x as i64) < center_width {
                            -center_width + (*x as i64)
                        } else {
                            (*x as i64) - center_width
                        };
                        let current_y: i64 = if (*y as i64) < center_height {
                            -center_height + (*y as i64)
                        } else {
                            (*y as i64) - center_height
                        };

                        let new_x = (current_y + center_height) as u32;
                        let new_y = (-current_x + center_width) as u32;

                        new_sprite_data.insert(Point::new(new_x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
            Rotation::Right => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .for_each(|(Point { x, y }, pixel)| {
                        let current_x: i64 = if (*x as i64) < center_width {
                            -center_width + (*x as i64)
                        } else {
                            (*x as i64) - center_width
                        };
                        let current_y: i64 = if (*y as i64) < center_height {
                            -center_height + (*y as i64)
                        } else {
                            (*y as i64) - center_height
                        };

                        let new_x = (-current_y + center_height) as u32;
                        let new_y = (current_x + center_width) as u32;

                        new_sprite_data.insert(Point::new(new_x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
            Rotation::UpsideDown => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .for_each(|(Point { x, y }, pixel)| {
                        let current_y: i64 = if (*y as i64) < center_height {
                            -center_height + (*y as i64)
                        } else {
                            (*y as i64) - center_height
                        };

                        // let new_x = (current_y + center_height) as u32;
                        let new_y = (-current_y + center_width) as u32;

                        new_sprite_data.insert(Point::new(*x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
        }
    }

    fn build_default_sprite() -> HashMap<Point, Pixel> {
        let row = vec![
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
            Some(Color::WHITE),
        ];

        let mut shape: Vec<Vec<Option<Color>>> = Vec::new();
        for _ in 0..8 {
            shape.push(row.clone());
        }

        let mut pixels: HashMap<Point, Pixel> = HashMap::new();

        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                let color = shape[y][x];
                if color.is_some() {
                    let location = Point::new(x as u32, y as u32);
                    pixels.insert(location.clone(), Pixel::new(color.unwrap()));
                }
            }
        }

        pixels
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite::new(Sprite::build_default_sprite())
    }
}

///
/// Dimensions implementation
///
impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Dimensions { width, height }
    }
}
