use std::time::{Duration, SystemTime};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use pacman::game_state::*;
use pacman::player_input::*;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

///
/// ===== Main Program ======
///

pub fn main() {

    playground();
    start();
}

pub fn playground() {

}

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("PacMan (Maybe Snake (idk rn))", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = GameState::init(WINDOW_WIDTH, WINDOW_HEIGHT);

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
                    Keycode::I => player_input = Some(PlayerInput::KeyDown(Key::I)),
                    Keycode::J => player_input = Some(PlayerInput::KeyDown(Key::J)),
                    Keycode::K => player_input = Some(PlayerInput::KeyDown(Key::K)),
                    Keycode::L => player_input = Some(PlayerInput::KeyDown(Key::L)),
                    Keycode::Num1 => player_input = Some(PlayerInput::KeyDown(Key::Num1)),
                    Keycode::Num2 => player_input = Some(PlayerInput::KeyDown(Key::Num2)),
                    Keycode::Num3 => player_input = Some(PlayerInput::KeyDown(Key::Num3)),
                    Keycode::Num4 => player_input = Some(PlayerInput::KeyDown(Key::Num4)),
                    Keycode::Num5 => player_input = Some(PlayerInput::KeyDown(Key::Num5)),
                    _ => {}
                },
                _ => {}
            }
        }

        game_state.tick(player_input);
        game_state.render(&mut canvas);

        canvas.present();

        // Log how long the frame processing took
        let elapsed = now.elapsed().unwrap().as_nanos();

        let percent_time_to_process = (elapsed as f64 / frame_duration as f64) * 100_f64;
        println!("Percent of frame to process: {}%", percent_time_to_process);
        println!("Elapsed time: {}", elapsed);

        let remaining_frame_duration =
            if elapsed > frame_duration as u128 { 0 } else { frame_duration - u32::try_from(elapsed).unwrap_or(0) };
        ::std::thread::sleep(Duration::new(0, remaining_frame_duration));
        // ::std::thread::sleep(Duration::new(0, frame_duration));
    }
}
