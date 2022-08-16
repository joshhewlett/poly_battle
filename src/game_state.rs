use crate::game_objects::*;
use crate::player_input::{Key, PlayerInput};
use crate::structs::*;
use crate::traits::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::time::SystemTime;
use rand::Rng;
use sdl2::libc::iconv;

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
    pub boundary_map: HashMap<Point, Vec<(GameObjectType, Pixel)>>,
    pub map: HashMap<Point, Vec<(GameObjectType, Pixel)>>,
    player: Player,
    coins: Vec<Coin>,
}

impl GameState {
    pub fn init(map_width: usize, map_height: usize) -> Self {
        // Init map state
        let mut map = HashMap::new();

        let boundary = Boundary::new();
        let boundaries: Vec<&dyn Drawable> = vec![&boundary];
        let boundary_map = GameState::convert_drawables_to_pixel_map(&boundaries);

        Self {
            map_width,
            map_height,
            boundary_map,
            map,
            player: Player::default(), // TODO: Create new non-default player with specific position
            coins: vec![Coin::default()],
        }
    }

    pub fn tick(&mut self, event: Option<PlayerInput>) {
        // Input
        if event.is_some() {
            match event.unwrap() {
                PlayerInput::KeyDown(key) => match key {
                    Key::W | Key::I => self.player.change_direction(Direction::Up),
                    Key::A | Key::J => self.player.change_direction(Direction::Left),
                    Key::S | Key::K => self.player.change_direction(Direction::Down),
                    Key::D | Key::L => self.player.change_direction(Direction::Right),
                },
            }
            println!("Player direction: {:?}", self.player.direction());
        }

        // Tick frame
        self.player.tick();

        // Update game state
        self.update_game_state();

        // --- Physics
        // Check for collisions
        self.check_for_collisions_with_player_and_coins();

        // Spawn coin if no other coin exists
        if self.coins.is_empty() {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.map_width) as i32;
            let y = rng.gen_range(0..self.map_height) as i32;

            self.coins.push(Coin::new(Point::new(x, y)));
        }
    }

    fn check_for_collisions_with_player_and_coins(&mut self) {
        // Did player collide with a coin?
        // Destroy coin
        let coin_ids: HashSet<i32> = self
            .map
            .iter()
            // Find all points that have a Player and at least one coin
            .filter(|(key, point_vec)| {
                point_vec.len() > 1
                    && point_vec.iter().any(|point| match point.0 {
                    GameObjectType::Player => true,
                    _ => false,
                })
                    && point_vec.iter().any(|point| match point.0 {
                    GameObjectType::Coin(_) => true,
                    _ => false,
                })
            })
            // Convert Map<Point, Vec<(GameObjectType, Pixel)> to Vec<Coin IDs>
            .flat_map(|(key, point_vec)| {
                point_vec
                    .iter()
                    .filter(|(game_object_type, _)| match game_object_type {
                        GameObjectType::Coin(_) => true,
                        _ => false,
                    })
                    .map(|(coin, _)| match coin {
                        GameObjectType::Coin(id) => id.clone(),
                        _ => panic!("Found non-coin object where I shouldn't"),
                    })
            })
            .collect::<HashSet<i32>>();

        coin_ids
            .iter()
            .for_each(|coin_id| self.collect_coin(coin_id.to_owned()));
    }

    fn collect_coin(&mut self, coin_id: i32) {
        println!("Collected coin: {}", coin_id);
        let index_opt = self
            .coins
            .iter()
            .position(|coin| coin.id() == coin_id)
            .expect(format!("Coin (ID: {}) not found", coin_id).as_str());

        self.coins.remove(index_opt);
        self.player.increment_coin_count();

        println!("Player coin count: {}", self.player.coin_count());
    }

    fn update_game_state(&mut self) {
        self.clear_map();

        // Last element in the drawables map will be at the forefront. Technically, the player
        // should be added last, but it's not an issue right now
        // let mut drawables: Vec<&dyn Drawable> = vec![&self.boundaries[0], &self.player];
        let mut drawables: Vec<&dyn Drawable> = vec![&self.player];
        self.coins.iter().for_each(|c| drawables.push(c));

        self.map = GameState::convert_drawables_to_pixel_map(&drawables);
    }

    fn convert_drawables_to_pixel_map(drawables: &Vec<&dyn Drawable>) -> HashMap<Point, Vec<(GameObjectType, Pixel)>> {
        let mut map: HashMap<Point, Vec<(GameObjectType, Pixel)>> = HashMap::new();
        for drawable in drawables {
            let entity_position: &Point = drawable.position();
            let entity_shape: &Shape = drawable.shape();
            let entity_type: GameObjectType = drawable.game_object_type();

            entity_shape
                .shape_data()
                .iter()
                .for_each(|(pixel_location, pixel)| {
                    let absolute_pos = Point::new(
                        entity_position.x + pixel_location.x,
                        entity_position.y + pixel_location.y,
                    );

                    if !map.contains_key(&absolute_pos) {
                        map.insert(absolute_pos.clone(), Vec::new());
                    }

                    map.get_mut(&absolute_pos).unwrap().push((
                        entity_type.clone(),
                        Pixel::new(absolute_pos.clone(), pixel.color.clone()),
                    ));
                })
        }

        map
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        fn render_map(map: &HashMap<Point, Vec<(GameObjectType, Pixel)>>, canvas: &mut WindowCanvas) {
            map.iter().for_each(|(point, pixels)| {
                pixels.iter().for_each(|pixel| {
                    canvas.set_draw_color(pixel.1.color);
                    canvas.draw_point(pixel.1.location).unwrap();
                })
            });
        }

        render_map(&self.boundary_map, canvas);
        render_map(&self.map, canvas);
    }

    fn clear_map(&mut self) {
        self.map.clear();
    }
}
