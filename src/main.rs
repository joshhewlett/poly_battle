use std::collections::HashSet;
use std::time::Duration;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static CENTER_X: i32 = (WINDOW_WIDTH / 2) as i32;
static CENTER_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

///
/// Shape definition
///
struct Shape {
    shape: Vec<Vec<Option<Color>>>, // TODO: Consider renaming this
}

// TODO: Should Shape implement Iterable?
impl Shape {
    fn new(shape: Vec<Vec<Option<Color>>>) -> Self {
        Shape { shape }
    }

    // fn shape_data(&self) -> &Vec<Vec<Option<Color>>> {
    //     &self.shape
    // }

    fn get_pixel(&self, x: usize, y: usize) -> &Option<Color> {
        &self.shape[y][x]
    }

    fn width(&self) -> usize {
        self.shape[0].len()
    }

    fn height(&self) -> usize {
        self.shape.len()
    }
}

impl Default for Shape {
    fn default() -> Self {
        let shape: Vec<Vec<Option<Color>>> = vec![
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
            vec![Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE),
                 Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE), Some(Color::WHITE)],
        ];

        Shape::new(shape)
    }
}

///
/// Drawable definition
///
trait Drawable {
    fn position(&self) -> &Point;
    fn set_position(&mut self, point: Point);
    fn shape(&self) -> &Shape;

    // TODO: rename and implement memoization somehow
    fn get_active_pixels(&self) -> Vec<(Point, &Option<Color>)> {
        let position: &Point = &self.position();
        let x_origin = position.x;
        let y_origin = position.y;

        let shape: &Shape = &self.shape();

        let mut result: Vec<(Point, &Option<Color>)> = Vec::new();

        for y_ptr in 0..shape.height() {
            for x_ptr in 0..shape.width() {
                let pixel: &Option<Color> = shape.get_pixel(x_ptr, y_ptr);

                if pixel.is_some() {
                    let point = Point::new(
                        x_origin + i32::try_from(x_ptr).unwrap(),
                        y_origin + i32::try_from(y_ptr).unwrap(),
                    );
                    result.push((point, pixel));
                }
            }
        }

        result
    }

    fn draw(&self, canvas: &mut WindowCanvas) {
        let pixels: Vec<(Point, &Option<Color>)> = self.get_active_pixels();

        pixels.iter().for_each(|pixel| {
            canvas.set_draw_color(pixel.1.unwrap());
            canvas.draw_point(pixel.0).unwrap();
        });
    }
}

///
/// Collidable definition
///
trait Collidable: Drawable {
    fn has_collided(&self, other: &dyn Collidable) -> bool {
        let common_points: HashSet<Point> = self
            .get_active_pixels()
            .iter()
            .map(|pixel| pixel.0)
            .collect();

        other
            .get_active_pixels()
            .iter()
            .map(|p| p.0)
            .any(|p| common_points.contains(&p))
    }
}

///
/// Moveable definition
///
trait Moveable: Drawable {
    fn direction(&self) -> &Direction;
    fn change_direction(&mut self, new_direction: Direction);

    fn apply_movement(&mut self) {

        let new_position: Point;
        match &self.direction() {
            Direction::Up => {
                new_position = Point::new(self.position().x, self.position().y - 1);
            }
            Direction::Down => {
                new_position = Point::new(self.position().x, self.position().y + 1);
            }
            Direction::Left => {
                new_position = Point::new(self.position().x - 1, self.position().y);
            }
            Direction::Right => {
                new_position = Point::new(self.position().x + 1, self.position().y);
            }
        }

        self.set_position(new_position);
    }
}

///
/// ===== Entities =====
///
/// Player definition
///
struct Player {
    position: Point,
    shape: Shape,
    current_direction: Direction,
}

impl Player {
    fn new(position: Point) -> Self {
        Player {
            position,
            shape: Shape::default(),
            current_direction: Direction::default(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point::new(CENTER_X, CENTER_Y))
    }
}

impl Drawable for Player {
    fn position(&self) -> &Point {
        &self.position
    }

    fn set_position(&mut self, new_position: Point) {
        self.position = new_position;
    }

    fn shape(&self) -> &Shape {
        &self.shape
    }
}

impl Collidable for Player {}

impl Moveable for Player {
    fn direction(&self) -> &Direction {
        &self.current_direction
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.current_direction = new_direction;
    }
}

///
/// Coin definition
///

struct Coin {
    position: Point,
    shape: Shape,
}

impl Coin {
    fn new(position: Point) -> Self {
        Coin {
            position,
            shape: Shape::default(),
        }
    }
}

impl Default for Coin {
    fn default() -> Self {
        Coin::new(Point::new(CENTER_X + 50, CENTER_Y + 50))
    }
}

// TODO: This is probably a good use of #derive!
impl Drawable for Coin {
    fn position(&self) -> &Point {
        &self.position
    }

    fn set_position(&mut self, new_position: Point) {
        self.position = new_position;
    }

    fn shape(&self) -> &Shape {
        &self.shape
    }
}

impl Collidable for Coin {}

///
/// ===== Main Program ======
///
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("PacMan (Maybe Snake (idk rn))", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = Player::default();
    let coin = Coin::default();

    let fps = 60;
    let frame_duration = 1_000_000_000u32 / fps;
    'running: loop {
        // let now = SystemTime::now();

        // TODO: Test if event stack pushes more than once a frame
        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    player.change_direction(Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    player.change_direction(Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    player.change_direction(Direction::Down);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    player.change_direction(Direction::Right);
                }
                _ => {}
            }
        }

        // Set background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // The rest of the game loop goes here...
        player.draw(&mut canvas);

        if player.has_collided(&coin) {
            println!("Player collected coin!");
            break 'running;
        } else {
            coin.draw(&mut canvas);
        }

        player.apply_movement();

        canvas.present();

        // Log how long the frame processing took
        // let elapsed = now.elapsed().unwrap().as_nanos();
        // let percent_time_to_process = (elapsed as f64 / frame_duration as f64) * 100 as f64;
        // println!("Percent of frame to process: {}%", percent_time_to_process);
        // println!("Elapsed time: {}", elapsed);
        // let remaining_frame_duration =
        //     if elapsed > frame_duration as u128 { 0 } else { frame_duration - u32::try_from(elapsed).unwrap_or(0) };
        // ::std::thread::sleep(Duration::new(0, remaining_frame_duration));
        ::std::thread::sleep(Duration::new(0, frame_duration));
    }
}
