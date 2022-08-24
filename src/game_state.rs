use crate::game_objects::*;
use crate::performance_tracking::PerformanceTracker;
use crate::player_input::{Key, PlayerInput};
use crate::structs::*;
use crate::traits::*;
use rand::Rng;
use sdl2::render::WindowCanvas;
use std::collections::{HashMap, HashSet};

struct GameMapDimensions {
    pub width: u32,
    pub height: u32,
    pub center: Point,
    pub origin: Point,
}

impl GameMapDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            center: Point::new(width / 2, height / 2),
            origin: Point::new(0, 0),
        }
    }
}

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
    map_dimensions: GameMapDimensions,
    boundary: Boundary,
    map: HashMap<Point, Vec<(GameObjectType, Pixel)>>,
    player: Player,
    coins: Vec<Coin>,
}

impl GameState {
    pub fn init(map_width: u32, map_height: u32) -> Self {
        // Create map dimensions
        let map_dimensions = GameMapDimensions::new(map_width, map_height);

        // Create boundaries
        let boundary = Boundary::new(map_dimensions.width, map_dimensions.height);

        // Create player
        let player = Player::new(map_dimensions.center.clone());

        // Create initial coin
        let coin_origin = Point::new(map_dimensions.origin.x + 100, map_dimensions.origin.y + 100);
        let coins = vec![Coin::new(coin_origin)];

        Self {
            map_dimensions,
            boundary,
            map: HashMap::new(),
            player,
            coins,
        }
    }

    pub fn tick(&mut self, event: Option<PlayerInput>) {
        //////// Handle Input //////
        if event.is_some() {
            match event.unwrap() {
                PlayerInput::KeyDown(key) => match key {
                    Key::W | Key::I => self.player.change_direction(Direction::Up),
                    Key::A | Key::J => self.player.change_direction(Direction::Left),
                    Key::S | Key::K => self.player.change_direction(Direction::Down),
                    Key::D | Key::L => self.player.change_direction(Direction::Right),
                    Key::Num1 => self.player.change_speed(1),
                    Key::Num2 => self.player.change_speed(2),
                    Key::Num3 => self.player.change_speed(3),
                    Key::Num4 => self.player.change_speed(4),
                    Key::Num5 => self.player.change_speed(5),
                },
            }
            println!("Player direction: {:?}", self.player.direction());
        }

        ////// Game Logic //////

        //// Tick GameObjects
        self.player.tick();

        //// Handle collisions
        // If player collides with boundary/wall, return to original position
        self.handle_collisions_with_boundary();

        // If player collides with coin, "collect" coin
        self.handle_collisions_with_coins();

        //// Convert to map
        self.update_game_state();

        // Spawn coin if no other coin exists
        if self.coins.is_empty() {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.map_dimensions.width) as u32;
            let y = rng.gen_range(0..self.map_dimensions.height) as u32;

            self.coins.push(Coin::new(Point::new(x, y)));
        }
    }

    fn handle_collisions_with_boundary(&mut self) {
        // Check for player and boundary collisions
        for player_point in self.player.effective_points() {
            // If player collides with boundary, set the player's origin to it's previous position
            if self.boundary.sprite().pixels().contains_key(player_point) {
                let prev_player_origin = self.player.prev_origin_unchecked();
                self.player.set_origin(&prev_player_origin.clone());
                break;
            }
        }
    }

    fn handle_collisions_with_coins(&mut self) {
        let coins_to_collect: Vec<u32> = self
            .coins
            .iter()
            .filter(|coin| coin.has_collided_with(Box::new(&self.player)))
            .map(|coin| coin.id())
            .collect();

        for coin_id in coins_to_collect {
            self.collect_coin(coin_id)
        }
    }

    fn collect_coin(&mut self, coin_id: u32) {
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
        let mut drawables: Vec<&dyn GameObject> = vec![&self.player];
        self.coins.iter().for_each(|c| drawables.push(c));

        self.map = GameState::convert_drawables_to_pixel_map(&drawables);
    }

    fn convert_drawables_to_pixel_map(
        drawables: &Vec<&dyn GameObject>,
    ) -> HashMap<Point, Vec<(GameObjectType, Pixel)>> {
        let mut map: HashMap<Point, Vec<(GameObjectType, Pixel)>> = HashMap::new();
        for drawable in drawables {
            let entity_position: &Point = drawable.origin();
            let entity_shape: &Sprite = drawable.sprite();
            let entity_type: &GameObjectType = drawable.game_object_type();

            entity_shape
                .pixels()
                .iter()
                .for_each(|(pixel_location, pixel)| {
                    let absolute_pos = Point::new(
                        entity_position.x + pixel_location.x,
                        entity_position.y + pixel_location.y,
                    );

                    if !map.contains_key(&absolute_pos) {
                        map.insert(absolute_pos.clone(), Vec::new());
                    }

                    map.get_mut(&absolute_pos)
                        .unwrap()
                        .push((entity_type.clone(), Pixel::new(&pixel.color)));
                })
        }

        map
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        fn render_map(map: &HashMap<Point, Pixel>, canvas: &mut WindowCanvas) {
            // Try multi-threading this?
            map.iter().for_each(|(point, pixel)| {
                canvas.set_draw_color(pixel.color);

                let canvas_point = sdl2::rect::Point::new(point.x as i32, point.y as i32);
                canvas.draw_point(canvas_point).unwrap();
            });
        }

        render_map(&self.boundary.sprite().pixels(), canvas);

        let normalized_map = self
            .map
            .iter()
            .map(|(point, pixels)| {
                (
                    point.to_owned(),
                    pixels.to_owned().last().unwrap().1.clone(),
                )
            })
            .collect::<HashMap<Point, Pixel>>();

        render_map(&normalized_map, canvas);
    }

    fn clear_map(&mut self) {
        self.map.clear();
    }
}
