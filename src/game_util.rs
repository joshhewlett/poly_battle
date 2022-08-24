use crate::structs::{Point, Sprite};
use std::collections::HashSet;

pub fn calc_effective_points_for_sprite(sprite: &Sprite, origin: &Point) -> HashSet<Point> {
    sprite
        .pixels()
        .iter()
        .map(|(point, _pixel)| Point::new(origin.x + point.x, origin.y + point.y))
        .collect()
}

pub fn has_collided(
    effective_points_for_obj_1: &HashSet<Point>,
    effective_points_for_obj_2: &HashSet<Point>,
) -> bool {
    // Determine which is smaller and bigger. This allows the for-loop below to run more efficiently
    let smaller_object: &HashSet<Point>;
    let bigger_object: &HashSet<Point>;
    match effective_points_for_obj_1.len() < effective_points_for_obj_2.len() {
        true => {
            smaller_object = &effective_points_for_obj_1;
            bigger_object = &effective_points_for_obj_2;
        }
        false => {
            bigger_object = &effective_points_for_obj_1;
            smaller_object = &effective_points_for_obj_2;
        }
    }

    for point in smaller_object {
        if bigger_object.contains(point) {
            return true;
        }
    }
    false
}
