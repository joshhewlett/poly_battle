use crate::game_objects::*;
use crate::structs::*;
use crate::traits::*;
use crate::util::has_collided;
use rand::Rng;
use sdl2::render::WindowCanvas;
use std::collections::{HashMap, HashSet};

const FRAME_RATE: u8 = 60;
const MAX_FIRE_RATE_PER_SEC: u8 = 7;
const MIN_FRAMES_BETWEEN_SHOTS: u8 = FRAME_RATE / MAX_FIRE_RATE_PER_SEC;
const FREE_ROME_ENABLED: bool = true;

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
            center: Point::new((width / 2) as i32, (height / 2) as i32),
            origin: Point::new(0, 0),
        }
    }
}

///
/// Game State definition
///
pub struct Game {
    // Dimensions of the map
    map_dimensions: GameMapDimensions,
    // Player object
    player: Player,
    // Game world boundary
    boundary: Boundary,
    // Coins that exist in the world
    coins: Vec<Coin>,
    projectiles: Vec<Projectile>,
}

impl Game {
    pub fn init(map_width: u32, map_height: u32) -> Self {
        // Create map dimensions
        let map_dimensions = GameMapDimensions::new(map_width, map_height);

        // Create boundaries
        let boundary = Boundary::new(map_dimensions.width, map_dimensions.height);

        // Create player
        let mut player = Player::new(Point::new(
            (map_dimensions.width / 2) as i32,
            (map_dimensions.height - 50) as i32,
        ));

        player.change_direction(Direction::Stopped);

        if FREE_ROME_ENABLED {
            player.disable_rotation();
        }

        // Create initial coin
        let coin_origin = Point::new(map_dimensions.origin.x + 100, map_dimensions.origin.y + 100);
        let coins = vec![Coin::new(coin_origin)];

        Self {
            map_dimensions,
            boundary,
            player,
            coins,
            projectiles: Vec::new(),
        }
    }

    pub fn tick(&mut self, event: Option<PlayerInput>) {
        //////// Input //////
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
                    Key::SpaceBar => self.fire_projectile(),
                },
            }
            println!("Player direction: {:?}", self.player.direction());
        }

        ////// Game Logic //////

        //// Tick GameObjects
        self.player.tick();
        self.projectiles.iter_mut().for_each(|mut p| p.tick());

        //// Handle collisions
        // If player collides with boundary/wall, return to original position
        self.handle_collisions_with_boundary();

        // If projectile collides with boundary, destroy it
        self.handle_collisions_projectile_and_boundary();

        // If player projectile collides with coin, "collect" coin
        self.handle_collisions_projectiles_and_coins();

        //// Additional events
        // Spawn coin if no other coin exists
        if self.coins.is_empty() {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.map_dimensions.width) as i32;
            let y = rng.gen_range(0..self.map_dimensions.height) as i32;

            self.coins.push(Coin::new(Point::new(x, y)));
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        fn render_map(pixels: &HashMap<Point, Pixel>, canvas: &mut WindowCanvas) {
            // Try multi-threading this?
            for (point, pixel) in pixels {
                let color = pixel.color;
                canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                    color.r, color.g, color.b, color.a,
                ));

                let canvas_point = sdl2::rect::Point::new(point.x as i32, point.y as i32);
                canvas.draw_point(canvas_point).unwrap();
            }
        }

        self.all_game_objects()
            .iter()
            .for_each(|obj| render_map(obj.effective_pixels(), canvas))
    }

    fn handle_collisions_with_boundary(&mut self) {
        // Check for player and boundary collisions
        for player_point in self.player.effective_points() {
            // If player collides with boundary, set the player's origin to it's previous position
            if self.boundary.sprite().pixels().contains_key(player_point) {
                let prev_player_origin = self.player.prev_origin_unchecked();
                self.player.set_origin(prev_player_origin);
                break;
            }
        }
    }

    fn handle_collisions_projectile_and_boundary(&mut self) {
        fn did_projectile_collide_with_boundary(
            boundary_points: &HashMap<Point, Pixel>,
            projectile: &Projectile,
        ) -> Option<u32> {
            for proj_point in projectile.effective_points() {
                if boundary_points.contains_key(proj_point) {
                    return Some(projectile.id());
                }
            }
            None
        }

        let projectiles_to_destroy: Vec<u32> = self
            .projectiles
            .iter()
            .map(|p| did_projectile_collide_with_boundary(self.boundary.effective_pixels(), p))
            .filter(|p_id| p_id.is_some())
            .map(|p_id| p_id.unwrap())
            .collect();

        for projectile_id in projectiles_to_destroy {
            self.destroy_projectile(projectile_id);
        }
    }

    fn handle_collisions_projectiles_and_coins(&mut self) {
        // It doesn't matter what projectile collided with a coin. So we should just loop through
        // the projectile list once and create a single HashSet containing all projectile effective
        // points. This allows each coin to check for collision using a O(n) operation (instead of O(n * m))
        let mut all_projectile_points: HashSet<Point> = HashSet::new();
        &self
            .projectiles
            .iter()
            .flat_map(|projectile| projectile.effective_points())
            .for_each(|point| {
                all_projectile_points.insert(*point);
            });

        let coins_to_collect: Vec<u32> = self
            .coins
            .iter()
            .filter(|coin| has_collided(coin.effective_points(), &all_projectile_points))
            .map(|coin| coin.id())
            .collect();

        for coin_id in coins_to_collect {
            self.collect_coin(coin_id)
        }
    }

    fn fire_projectile(&mut self) {
        if MIN_FRAMES_BETWEEN_SHOTS as u32 > self.player.frames_since_last_shot() {
            return;
        }

        let projectile_direction = match self.player.rotation() {
            Rotation::Up => Direction::Up,
            Rotation::Right => Direction::Right,
            Rotation::Down => Direction::Down,
            Rotation::Left => Direction::Left,
        };

        self.projectiles.push(Projectile::new(
            self.player.origin(),
            projectile_direction,
            self.player.rotation(),
        ));
        self.player.reset_frames_since_last_shot();
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

    fn destroy_projectile(&mut self, projectile_id: u32) {
        let index_opt = self
            .projectiles
            .iter()
            .position(|projectile| projectile.id() == projectile_id)
            .expect(format!("Projectile (ID: {}) not found", projectile_id).as_str());

        self.projectiles.remove(index_opt);
    }

    fn all_game_objects(&self) -> Vec<&dyn GameObject> {
        let mut all_game_objects: Vec<&dyn GameObject> = vec![&self.boundary];
        self.coins
            .iter()
            .for_each(|coin| all_game_objects.push(coin));
        self.projectiles
            .iter()
            .for_each(|projectile| all_game_objects.push(projectile));
        all_game_objects.push(&self.player);

        all_game_objects
    }
}
