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
use pacman::player_input::*;
use pacman::structs::*;
use pacman::traits::*;

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

    let mut game_state = GameState::init(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);

    let fps = 60;
    let frame_duration = 1_000_000_000u32 / fps;
    'running: loop {
        let now = SystemTime::now();

        // Set background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Check for events
        let mut player_input: Option<PlayerInput> = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                // Player controls
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::W => player_input = Some(PlayerInput::KeyDown(Key::W)),
                    Keycode::A => player_input = Some(PlayerInput::KeyDown(Key::A)),
                    Keycode::S => player_input = Some(PlayerInput::KeyDown(Key::S)),
                    Keycode::D => player_input = Some(PlayerInput::KeyDown(Key::D)),
                    _ => {}
                },
                _ => {}
            }
        }

        // player.apply_movement();
        game_state.tick(player_input);
        game_state.render(&mut canvas);

        canvas.present();

        // Log how long the frame processing took
        let elapsed = now.elapsed().unwrap().as_nanos();
        let percent_time_to_process = (elapsed as f64 / frame_duration as f64) * 100 as f64;
        // println!("Percent of frame to process: {}%", percent_time_to_process);
        // println!("Elapsed time: {}", elapsed);
        // let remaining_frame_duration =
        //     if elapsed > frame_duration as u128 { 0 } else { frame_duration - u32::try_from(elapsed).unwrap_or(0) };
        // ::std::thread::sleep(Duration::new(0, remaining_frame_duration));
        ::std::thread::sleep(Duration::new(0, frame_duration));
    }
}
