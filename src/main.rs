use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use poly_battle::game::*;
use poly_battle::structs::*;
use poly_battle::util::*;

static GAME_TITLE: &'static str = "PolyBattle";
static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

///
/// ===== Main Program ======
///

pub fn main() {
    playground();
    start();
}

pub fn playground() {}

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(GAME_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = Game::init(WINDOW_WIDTH, WINDOW_HEIGHT);

    let fps = 60;
    let frame_duration = 1_000_000_000u32 / fps;

    'running: loop {
        let mut performance_tracker = PerformanceTracker::new();

        // Set background
        performance_tracker.measure_unit_of_work("draw_background", || {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
        });

        // Check for events
        performance_tracker.start_unit_of_work("get_input");
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
        performance_tracker.end_unit_of_work("get_input").unwrap();

        performance_tracker.measure_unit_of_work("game_tick", || {
            game_state.tick(player_input);
        });

        performance_tracker.measure_unit_of_work("render", || {
            game_state.render(&mut canvas);
        });

        performance_tracker.measure_unit_of_work("present_canvas", || {
            canvas.present();
        });

        // Output frame performance metrics
        let elapsed = performance_tracker.end().unwrap().as_nanos();
        println!("{}", performance_tracker);

        // Cap FPS to Frame Rate
        let remaining_frame_duration = if elapsed > frame_duration as u128 {
            0
        } else {
            frame_duration - u32::try_from(elapsed).unwrap_or(0)
        };
        std::thread::sleep(Duration::new(0, remaining_frame_duration));
    }
}
