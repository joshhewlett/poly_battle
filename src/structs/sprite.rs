use std::collections::HashMap;
use crate::structs::{Dimensions, Pixel, Point};
use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel as ImagePixel};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 255,
        }
    }

    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    pub const WHITE: Color = Color::RGB(255, 255, 255);
    pub const BLACK: Color = Color::RGB(0, 0, 0);
    pub const RED: Color = Color::RGB(255, 0, 0);
    pub const GREEN: Color = Color::RGB(0, 255, 0);
    pub const BLUE: Color = Color::RGB(0, 0, 255);
}

///
/// Sprite definition
///
#[derive(Debug)]
pub struct Sprite {
    dimensions: Dimensions,
    sprite_data: HashMap<Point, Pixel>,
}

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
            sprite_data,
        }
    }

    pub fn new_from_file(file_name: &str) -> Self {
        let img = ImageReader::open(file_name).unwrap().decode().unwrap();

        let img_width = img.width();
        let img_height = img.height();

        let mut sprite_data: HashMap<Point, Pixel> = HashMap::new();
        img.pixels()
            .filter(|(_x, _y, (rgba))|
                rgba[0] != 0 ||
                    rgba[1] != 0 ||
                    rgba[2] != 0 ||
                    rgba[3] != 0
            )
            .for_each(|(x, y, (rgba))| {
                sprite_data.insert(Point::new(x, y), Pixel::new(Color::RGBA(rgba[0], rgba[1], rgba[2], rgba[3])));
            });

        Self {
            dimensions: Dimensions::new(img_width, img_height),
            sprite_data,
        }
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }

    pub fn pixels(&self) -> &HashMap<Point, Pixel> {
        &self.sprite_data
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

        assert!(
            shape.len() < u32::MAX as usize,
            "Shape height larger than expected"
        );
        assert!(
            shape[0].len() < u32::MAX as usize,
            "Shape width larger than expected"
        );
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

// TODO delete this
impl Default for Sprite {
    fn default() -> Self {
        Sprite::new(Sprite::build_default_sprite())
    }
}