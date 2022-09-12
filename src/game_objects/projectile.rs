use std::collections::{HashMap, HashSet};
use crate::structs::{Dimensions, Direction, GameObjectType, Pixel, Point, Rotation, Sprite};
use crate::traits::{GameObject, Movable};
use crate::util::calc_effective_sprite_pixels;

pub const PROJECTILE_SPRITE_FILENAME: &'static str = "projectile_sprite";
static mut ID_COUNTER: u32 = 0;

pub struct Projectile {
    id: u32,
    game_object_type: GameObjectType,
    origin: Point,
    sprite: Sprite,
    effective_sprite_pixels: HashMap<Point, Pixel>,
    effective_sprite_points: HashSet<Point>,
    speed: u32,
    direction: Direction,
}

impl Projectile {
    pub fn new(origin: Point) -> Self {
        let sprite = Sprite::new_from_file(PROJECTILE_SPRITE_FILENAME);
        let (effective_sprite_pixels, effective_sprite_points) =
            calc_effective_sprite_pixels(&sprite, origin);

        Self {
            id: Self::id(),
            game_object_type: GameObjectType::Projectile,
            origin,
            sprite,
            effective_sprite_pixels,
            effective_sprite_points,
            direction: Direction::Up,
            speed: 1
        }
    }

    fn id() -> u32 {
        let id: u32;
        unsafe {
            id = ID_COUNTER;
            ID_COUNTER += 1;
        }

        id
    }
}

impl GameObject for Projectile {
    fn tick(&mut self) {
        self.apply_movement();

        let (effective_sprite_pixels, effective_sprite_points) =
            calc_effective_sprite_pixels(&self.sprite, self.origin);

        self.effective_sprite_pixels = effective_sprite_pixels;
        self.effective_sprite_points = effective_sprite_points;
    }

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
        self.origin = new_origin
    }

    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn sprite_dimensions(&self) -> Dimensions {
        *self.sprite.dimensions()
    }

    fn effective_points(&self) -> &HashSet<Point> {
        &self.effective_sprite_points
    }

    fn effective_pixels(&self) -> &HashMap<Point, Pixel> {
        &self.effective_sprite_pixels
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Projectile::new(Point::new(400, 300))
    }
}

impl Movable for Projectile {
    fn direction(&self) -> Direction {
        self.direction
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction
    }

    fn rotation(&self) -> Rotation {
        Rotation::None
    }

    fn change_rotation(&mut self, _new_rotation: Rotation) {
        // Do nothing
    }

    fn speed(&self) -> u32 {
        self.speed
    }

    fn prev_origin(&self) -> Option<Point> {
        None
    }

    fn set_prev_origin(&mut self, _current_origin: Point) {
        // Do nothing
    }
}