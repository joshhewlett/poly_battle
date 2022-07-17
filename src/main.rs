use std::time::{Duration, SystemTime};

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Keycode::P;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, WindowCanvas};

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

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
        Shape {
            shape,
        }
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
        ];

        Shape::new(shape)
    }
}

///
/// Drawable definition
///
trait Drawable: {
    fn position(&self) -> &Point;
    fn shape(&self) -> &Shape;

    fn draw(&self, canvas: &mut WindowCanvas) {
        let position: &Point = &self.position();
        let x_origin = position.x;
        let y_origin = position.y;

        let shape: &Shape = &self.shape();

        for y_ptr in 0..shape.height() {
            for x_ptr in 0..shape.width() {
                let pixel: &Option<Color> = shape.get_pixel(x_ptr, y_ptr);

                if pixel.is_some() {
                    &canvas.set_draw_color(pixel.unwrap());
                    &canvas.draw_point(Point::new(x_origin + i32::try_from(x_ptr).unwrap(), y_origin + i32::try_from(y_ptr).unwrap()));
                }
            }
        }
    }
}

///
/// Collidable definition
///
// TODO
trait Collidable {}

///
/// Collidable definition
///
// TODO

///
/// Moveable definition
///
trait Moveable : Collidable {
    fn direction(&self) -> &Direction;
    fn apply_movement(&mut self);
    fn change_direction(&mut self, new_direction: Direction);
}

///
/// Player definition
///
struct Player {
    position: Point,
    shape: Shape,
    current_direction: Direction,
}

impl Player {
    fn new(point: Point) -> Self {
        Player {
            position: point,
            shape: Shape::default(),
            current_direction: Direction::default(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point::new(0, 0))
    }
}

impl Drawable for Player {
    fn position(&self) -> &Point {
        &self.position
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

    fn apply_movement(&mut self) {
        // TODO: Implement default apply_movement method
        match &self.current_direction {
            Direction::Up => {
                self.position.y -= 1;
            }
            Direction::Down => {
                self.position.y += 1;
            }
            Direction::Left => {
                self.position.x -= 1;
            }
            Direction::Right => {
                self.position.x += 1;
            }
        }
    }
}

///
/// Main program
///
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("PacMan (Maybe Snake)", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = Player::default();

    let fps = 60;
    let frame_duration = 1_000_000_000u32 / fps;
    'running: loop {
        let now = SystemTime::now();

        // TODO: Test if event stack pushes more than once a frame
        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    &player.change_direction(Direction::Up);
                }
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    &player.change_direction(Direction::Left);
                }
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    &player.change_direction(Direction::Down);
                }
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    &player.change_direction(Direction::Right);
                }
                _ => {}
            }
        }

        // Set background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // The rest of the game loop goes here...
        // apply_direction(&mut player_position, &direction);
        // draw_square(&mut canvas, Color::WHITE, &player_position, 20);
        // draw_triangle(&mut canvas, Color::WHITE, &player_position, 60);

        &player.draw(&mut canvas);

        &player.apply_movement();

        canvas.present();

        // Log how long the frame processing took
        let elapsed = now.elapsed().unwrap().as_nanos();
        let percent_time_to_process = (elapsed as f64 / frame_duration as f64) * 100 as f64;
        println!("Percent of frame to process: {}%", percent_time_to_process);
        println!("Elapsed time: {}", elapsed);
        let remaining_frame_duration =
            if elapsed > frame_duration as u128 { 0 } else { frame_duration - u32::try_from(elapsed).unwrap_or(0) };
        ::std::thread::sleep(Duration::new(0, remaining_frame_duration));
    }
}

//
// fn draw_square(canvas: &mut WindowCanvas, color: Color, origin: &Point, width: u32) {
//     &canvas.set_draw_color(color);
//
//
//     for y_pos in origin.y..(origin.y + width as i32) {
//         for x_pos in origin.x..(origin.x + width as i32) {
//             &canvas.draw_point(Point::new(x_pos, y_pos));
//         }
//     }
// }
//
// fn draw_triangle(canvas: &mut WindowCanvas, color: Color, origin: &Point, height: u32) {
//     &canvas.set_draw_color(color);
//
//     let mut line_width = 1;
//
//     for y_pos in origin.y..(origin.y + height as i32) {
//         for x_pos in (origin.x - (line_width / 2))..(origin.x + (line_width / 2)) {
//             &canvas.draw_point(Point::new(x_pos, y_pos));
//         }
//
//         if (y_pos % 2 == 0) {
//             line_width += 2;
//         }
//     }
// }