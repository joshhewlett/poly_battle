use crate::structs::{Color, Pixel, Point, Rotation};
use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel as ImagePixel};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use serde::Deserialize;

pub const SPRITE_RESOURCE_DIR: &'static str = "resources/sprites/";

///
/// Structs
///
#[derive(Debug)]
pub struct Sprite {
    dimensions: Dimensions,
    origin: Point,
    original_sprite_data: HashMap<Point, Pixel>,
    sprite_data: HashMap<Point, Pixel>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub struct SpriteMetadata {
    pub dimensions: Dimensions,
    pub origin: Point,
}

///
/// Sprite implementation
///
impl Sprite {
    pub fn new(sprite_data: HashMap<Point, Pixel>) -> Self {
        let (mut x_min, mut x_max): (i32, i32) = (0, 0);
        let (mut y_min, mut y_max): (i32, i32) = (0, 0);

        // Find width and height of sprite
        for (point, _pixel) in sprite_data.iter() {
            if point.x < x_min {
                x_min = point.x;
            } else if point.x > x_max {
                x_max = point.x
            }
            if point.y < y_min {
                y_min = point.y
            } else if point.y > y_max {
                y_max = point.y
            }
        }
        let width: u32 = u32::try_from(x_max - x_min).unwrap();
        let height: u32 = u32::try_from(y_max - y_min).unwrap();

        Sprite {
            dimensions: Dimensions::new(width, height),
            origin: Point::new(0, 0),
            original_sprite_data: sprite_data.clone(),
            sprite_data,
        }
    }

    pub fn new_from_file(file_name: &str) -> Self {

        let metadata_filename = SPRITE_RESOURCE_DIR.to_owned() + file_name + ".json";
        let image_filename = SPRITE_RESOURCE_DIR.to_owned() + file_name + ".png";

        // Retrieve metadata
        let metadata_file = File::options()
            .read(true)
            .open(metadata_filename)
            .unwrap();

        let mut metadata_contents = String::new();
        BufReader::new(metadata_file)
            .read_to_string(&mut metadata_contents)
            .unwrap();

        // Deserialize metadata
        let metadata: SpriteMetadata = serde_json::from_str(&metadata_contents)
            .expect("Error deserializing sprite metadata");
        let dimensions = metadata.dimensions;
        let origin = metadata.origin;

        // Decode image info
        let img = ImageReader::open(image_filename)
            .unwrap()
            .decode()
            .unwrap();

        let mut sprite_data: HashMap<Point, Pixel> = HashMap::new();
        img.pixels()
            // Filter out points that contain no info
            .filter(|(_x, _y, (rgba))| rgba[0] != 0 || rgba[1] != 0 || rgba[2] != 0 || rgba[3] != 0)
            // For each point, translate it relative to the metadata.origin
            .for_each(|(x, y, (rgba))| {
                let translated_x = (x as i32) - origin.x;
                let translated_y = (y as i32) - origin.y;

                sprite_data.insert(
                    Point::new(translated_x, translated_y),
                    Pixel::new(Color::RGBA(rgba[0], rgba[1], rgba[2], rgba[3])),
                );
            });

        Self {
            dimensions,
            origin,
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

    pub fn rotate_sprite_around_origin(&mut self, rotation: Rotation) {
        // Represents the center-point of the image to rotate
        // The reason for this is that all other logic assumes the origin is at (0, 0) of the
        // image map. For rotation however, we need to translate the points relative to the center
        // of the image. For example, in a 4x4 grid, the center point would be (2, 2) and the point
        // (0, 0) would now be represented as (-2, -2)
        // let center_width: u32 = self.dimensions.width / 2;
        // let center_height: u32 = self.dimensions.height / 2;

        match rotation {
            Rotation::None => self.sprite_data = self.original_sprite_data.clone(),
            Rotation::Left => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .for_each(|(point, pixel)| {
                        let new_x = point.y;
                        let new_y = -point.x;

                        new_sprite_data.insert(Point::new(new_x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
            Rotation::Right => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .for_each(|(point, pixel)| {
                        let new_x = -point.y;
                        let new_y = point.x;

                        new_sprite_data.insert(Point::new(new_x, new_y), *pixel);
                    });

                self.sprite_data = new_sprite_data;
            }
            Rotation::UpsideDown => {
                let mut new_sprite_data: HashMap<Point, Pixel> = HashMap::new();

                self.original_sprite_data
                    .iter()
                    .for_each(|(point, pixel)| {
                        let new_x = point.x;
                        let new_y = -point.y;

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

        assert!(shape.len() < i32::MAX as usize, "Shape height too large");
        assert!(shape[0].len() < i32::MAX as usize, "Shape width too large");
        for y in 0..shape.len() {
            for x in 0..shape[y].len() {
                let color = shape[y][x];
                if color.is_some() {
                    let location = Point::new(x as i32 , y as i32);
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
