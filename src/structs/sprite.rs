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
        // Represents the center-point of the image to rotate
        // The reason for this is that all other logic assumes the origin is at (0, 0) of the
        // image map. For rotation however, we need to translate the points relative to the center
        // of the image. For example, in a 4x4 grid, the center point would be (2, 2) and the point
        // (0, 0) would now be represented as (-2, -2)
        let center_width: u32 = self.dimensions.width / 2;
        let center_height: u32 = self.dimensions.height / 2;

        match rotation {
            Rotation::None => self.sprite_data = self.original_sprite_data.clone(),
            Rotation::Left => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .map(|(point, pixel)| {
                        (
                            Self::relocate_point_around_origin(
                                Point::new(center_width, center_height),
                                *point,
                            ),
                            pixel,
                        )
                    })
                    .for_each(|((translated_x, translated_y), pixel)| {
                        let new_x =
                            u32::try_from(translated_y + i32::try_from(center_height).unwrap())
                                .unwrap();
                        let new_y =
                            u32::try_from(-translated_x + i32::try_from(center_width).unwrap())
                                .unwrap();

                        new_sprite_data.insert(Point::new(new_x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
            Rotation::Right => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .map(|(point, pixel)| {
                        (
                            Self::relocate_point_around_origin(
                                Point::new(center_width, center_height),
                                *point,
                            ),
                            pixel,
                        )
                    })
                    .for_each(|((translated_x, translated_y), pixel)| {
                        let new_x =
                            u32::try_from(-translated_y + i32::try_from(center_height).unwrap())
                                .unwrap();
                        let new_y =
                            u32::try_from(translated_x + i32::try_from(center_width).unwrap())
                                .unwrap();

                        new_sprite_data.insert(Point::new(new_x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
            Rotation::UpsideDown => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .map(|(point, pixel)| {
                        (
                            Self::relocate_point_around_origin(
                                Point::new(center_width, center_height),
                                *point,
                            ),
                            pixel,
                        )
                    })
                    .for_each(|((translated_x, translated_y), pixel)| {
                        let new_x: u32 =
                            u32::try_from(translated_x + i32::try_from(center_width).unwrap())
                                .unwrap();
                        let new_y: u32 =
                            u32::try_from(-translated_y + i32::try_from(center_width).unwrap())
                                .unwrap();

                        new_sprite_data.insert(Point::new(new_x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
        }
    }

    fn relocate_point_around_origin(new_origin: Point, point: Point) -> (i32, i32) {
        let new_x: i32 = i32::try_from(point.x).unwrap() - i32::try_from(new_origin.x).unwrap();
        let new_y: i32 = i32::try_from(point.y).unwrap() - i32::try_from(new_origin.y).unwrap();

        (new_x, new_y)
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
