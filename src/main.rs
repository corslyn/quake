use std::time::Duration;

use crate::config::*;
use crate::render::*;
use pak::Pak;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

mod config;
mod pak;
mod palette;
mod render;

fn main() -> Result<(), String> {
    let pak = Pak::new("id1/PAK0.PAK").unwrap();
    let files = Pak::read_directory(&pak).unwrap();

    let palette = pak.find_file("gfx/palette.lmp").unwrap();

    let converted_palette = palette::convert_palette(&palette);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Birmingham Simulator", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        render(&mut canvas, &converted_palette);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 72)); // 72 fps
    }
    Ok(())
}
