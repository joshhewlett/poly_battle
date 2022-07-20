use sdl2::render::WindowCanvas;
use crate::traits::*;
use crate::structs::*;
use crate::game_objects::*;

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
    pub map: Vec<Vec<Vec<Pixel>>>,
    player: Player,
    coins: Vec<Coin>,
}

impl GameState {
    pub fn init(map_width: usize, map_height: usize) -> Self {

        // Init map state
        // TODO: Optimize this?
        let mut map: Vec<Vec<Vec<Pixel>>> = Vec::new();
        for y in 0..map_height {
            map.push(Vec::new()); // Create row

            for x in 0..map_width {
                map[y].push(Vec::new()); // Create column in individual row
                map[y][x] = Vec::new(); // Create Vec<Pixel> for each location in map
            }
        }

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

    pub fn tick(&mut self) {

        // TODO
        //   Where are you at with this?
        //   You need to figure out how to process players, "coins" and other objects, etc
        //   The issue is I can't cast a GameObject to a more specific type


        self.player.tick();
        self.update_map();

        //
        // self.state
        //     .iter()
        //     .for_each(|obj| {
        //         match obj.game_object_type() {
        //             GameObjectType::Player => {
        //                 self.process_player(obj);
        //             }
        //             _ => {
        //                 ()
        //             }
        //         }
        //     })
    }

    fn process_player(&mut self, player: &Player) {}

    fn update_map(&mut self) {
        self.clear_map();

        let mut drawables: Vec<&dyn Drawable> = vec![
            &self.player
        ];

        self.coins.iter()
            .for_each(|coin| drawables.push(coin));

        for drawable in drawables {
            drawable.get_active_pixels()
                .iter()
                .for_each(|pixel| {
                    self.map[pixel.location.y as usize][pixel.location.x as usize].push(pixel.to_owned());
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
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                for i in 0..self.map[y][x].len() {
                    let pixel = &self.map[y][x][i];

                    canvas.set_draw_color(pixel.color);
                    canvas.draw_point(pixel.location).unwrap();
                }
            }
        }
    }

    fn clear_map(&mut self) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                self.map[y][x] = Vec::new();
            }
        }
    }
}