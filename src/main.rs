use std::collections::HashSet;
use std::time::{Duration, SystemTime};

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use pacman::game_objects::*;
use pacman::game_state::*;
use pacman::traits::*;
use pacman::structs::*;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static CENTER_X: i32 = (WINDOW_WIDTH / 2) as i32;
static CENTER_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

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

    let mut game_state = GameState::init(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);

    let fps = 60;
    let frame_duration = 1_000_000_000u32 / fps;
    'running: loop {
        let now = SystemTime::now();

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
        // player.draw(&mut canvas);

        // if player.has_collided(&coin) {
        //     println!("Player collected coin!");
        //     break 'running;
        // } else {
        //     coin.draw(&mut canvas);
        // }

        // player.apply_movement();
        game_state.tick();
        game_state.render(&mut canvas);

        canvas.present();

        // Log how long the frame processing took
        let elapsed = now.elapsed().unwrap().as_nanos();
        let percent_time_to_process = (elapsed as f64 / frame_duration as f64) * 100 as f64;
        println!("Percent of frame to process: {}%", percent_time_to_process);
        println!("Elapsed time: {}", elapsed);
        // let remaining_frame_duration =
        //     if elapsed > frame_duration as u128 { 0 } else { frame_duration - u32::try_from(elapsed).unwrap_or(0) };
        // ::std::thread::sleep(Duration::new(0, remaining_frame_duration));
        ::std::thread::sleep(Duration::new(0, frame_duration));
    }
}
