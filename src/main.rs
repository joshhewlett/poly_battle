use std::time::{Duration, SystemTime};

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Keycode::P;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("PacMan (Maybe Snake)", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut line_width: i32 = 0;
    let mut count: i32 = 0;

    let fps = 60;
    let frame_duration = 1_000_000_000u32 / fps;
    'running: loop {
        let now = SystemTime::now();

        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Set background
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Calculate line width and height
        line_width = (line_width + 1) % WINDOW_WIDTH as i32;
        let line_height: i32 = ((WINDOW_HEIGHT as f32 / WINDOW_WIDTH as f32) * line_width as f32) as i32;

        // Draw line with width of 10
        // canvas.set_draw_color(Color::WHITE);
        // let start_point = Point::new(0, 0);
        // let end_point = Point::new(line_width, line_height);

        // for i in 0..10 {
        //     canvas.draw_line(start_point.offset(0, i), end_point.offset(0, i));
        // }

        // The rest of the game loop goes here...
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

fn draw_square(canvas: &mut WindowCanvas, color: Color, origin: Point, width: u32) {

    &canvas.set_draw_color(color);

    for y_pos in origin.y..(origin.y + width as i32) {
        for x_pos in origin.x..(origin.x + width as i32) {
            &canvas.draw_point(Point::new(x_pos, y_pos));
        }
    }
}