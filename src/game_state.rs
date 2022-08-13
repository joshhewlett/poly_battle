use std::collections::{HashMap, HashSet};
use std::ops::Index;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::traits::*;
use crate::structs::*;
use crate::game_objects::*;
use crate::player_input::{Key, PlayerInput};

/// Game State definition
///
/// map_width: Width of the map
/// map_height: Height of the map
/// map: Current state of the game map
///
/// game_objects: Vec<GameObject>
/// collision_rules: Vec<CollisionRule>
///
pub struct GameState {
    pub map_width: usize,
    pub map_height: usize,
    pub map: HashMap<Point, Vec<(GameObjectType, Pixel)>>,
    player: Player,
    coins: Vec<Coin>,
}

impl GameState {
    pub fn init(map_width: usize, map_height: usize) -> Self {

        // Init map state
        // TODO: Optimize this?
        // let mut map: Vec<Vec<Vec<Pixel>>> = Vec::new();
        // for y in 0..map_height {
        //     map.push(Vec::new()); // Create row
        //
        //     for x in 0..map_width {
        //         map[y].push(Vec::new()); // Create column in individual row
        //         map[y][x] = Vec::new(); // Create Vec<Pixel> for each location in map
        //     }
        // }
        let mut map = HashMap::new();

        // Quick development, add a couple default game objects
        //
        // let mut state: Vec<Box<dyn GameObject>> = vec![
        //     Box::new(Player::default()),
        //     Box::new(Coin::default())];
        Self {
            map_width,
            map_height,
            map,
            player: Player::default(),
            coins: vec![Coin::default()],
        }
    }

    pub fn tick(&mut self, event: Option<PlayerInput>) {

        // TODO
        //   Where are you at with this?
        //   You need to figure out how to process players, "coins" and other objects, etc
        //   The issue is I can't cast a GameObject to a more specific type

        // Input
        if event.is_some() {
            match event.unwrap() {
                PlayerInput::KeyDown(key) => {
                    match key {
                        Key::W => self.player.change_direction(Direction::Up),
                        Key::A => self.player.change_direction(Direction::Left),
                        Key::S => self.player.change_direction(Direction::Down),
                        Key::D => self.player.change_direction(Direction::Right)
                    }
                }
            }
            println!("Player direction: {:?}", self.player.direction());
        }

        // Tick frame
        self.player.tick();

        // Update game state
        self.update_map();

        // --- Physics
        // Check for collisions
        self.check_for_collisions();

    }

    fn process_player(&mut self, player: &Player) {}

    fn check_for_collisions(&mut self) {

        // Did player collide with a coin?
        // Destroy coin
        let coin_ids: HashSet<i32> = self.map.iter()
            // Find all points that have a Player and at least one coin
            .filter(|(key, point_vec)|
                point_vec.len() > 1 &&
                    point_vec
                        .iter()
                        .any(|point| match point.0 {
                            GameObjectType::Player => true,
                            _ => false,
                        }) &&
                    point_vec
                        .iter()
                        .any(|point| match point.0 {
                            GameObjectType::Coin(_) => true,
                            _ => false,
                        }))
            // Convert Map<Point, Vec<(GameObjectType, Pixel)> to Vec<Coin IDs>
            .flat_map(|(key, point_vec)|
                point_vec
                    .iter()
                    .filter(|(game_object_type, _)| match game_object_type {
                        GameObjectType::Coin(_) => true,
                        _ => false,
                    })
                    .map(|(coin, _)| match coin {
                        GameObjectType::Coin(id) => id.clone(),
                        _ => panic!("Found non-coin object where I shouldn't")
                    })
            )
            .collect::<HashSet<i32>>();

        coin_ids.iter().for_each(|coin_id| self.collect_coin(coin_id.to_owned()));
    }

    fn collect_coin(&mut self, coin_id: i32) {

        println!("Collected coin: {}", coin_id);
        let index_opt = self.coins
            .iter()
            .position(|coin| coin.id() == coin_id)
            .expect(format!("Coin (ID: {}) not found", coin_id).as_str());

        self.coins.remove(index_opt);
        self.player.increment_coin_count();

        println!("Player coin count: {}", self.player.coin_count());
    }

    fn update_map(&mut self) {
        self.clear_map();

        // Last element in the drawables map will be at the forefront. Technically, the player
        // should be added last, but it's not an issue right now
        let mut drawables: Vec<&dyn Drawable> = vec![
            &self.player
        ];
        self.coins.iter().for_each(|c| drawables.push(c));

        for drawable in drawables {
            let entity_position: &Point = drawable.position();
            let entity_shape: &Shape = drawable.shape();
            let entity_type: GameObjectType = drawable.game_object_type();

            entity_shape.shape_data().iter()
                .for_each(|(pixel_location, pixel)| {

                    // TODO: Account for position
                    let absolute_pos =
                        Point::new(
                            entity_position.x + pixel_location.x,
                            entity_position.y + pixel_location.y);

                    if !self.map.contains_key(&absolute_pos) {
                        self.map.insert(absolute_pos.clone(), Vec::new());
                    }

                    self.map.get_mut(&absolute_pos).unwrap().push(
                        (entity_type.clone(),
                         Pixel::new(
                             absolute_pos.clone(),
                             pixel.color.clone(), )));
                })
        }

        // TODO: Look into this error:
        //   "closure requires unique access to `self.map` but it is already borrowed"
        // for drawable in &self.drawables() {
        //
        //     drawable.get_active_pixels()
        //         .iter()
        //         .for_each(|pixel| {
        //             self.map[pixel.location.y as usize][pixel.location.x as usize].push(pixel.to_owned());
        //         })
        // }
    }

    fn drawables(&self) -> Vec<&dyn Drawable> {
        let mut drawables: Vec<&dyn Drawable> = vec![
            &self.player
        ];

        self.coins.iter()
            .for_each(|coin| drawables.push(coin));

        drawables
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {

        // TODO: Optimize this using HashSets
        // for y in 0..self.map_height {
        //     for x in 0..self.map_width {
        //         for i in 0..self.map[y][x].len() {
        //             let pixel = &self.map[y][x][i];
        //
        //             canvas.set_draw_color(pixel.color);
        //             canvas.draw_point(pixel.location).unwrap();
        //         }
        //     }
        // }

        self.map.iter()
            .for_each(|(point, pixels)| {
                pixels.iter().for_each(|pixel| {
                    canvas.set_draw_color(pixel.1.color);
                    canvas.draw_point(pixel.1.location).unwrap();
                })
            });
    }

    fn clear_map(&mut self) {
        self.map.clear();
        // for y in 0..self.map_height {
        //     for x in 0..self.map_width {
        //         self.map[y][x] = Vec::new();
        //     }
        // }
    }
}