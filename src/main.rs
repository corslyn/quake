use std::time::Duration;

use crate::config::*;
use crate::models::{Model, ModelHeader};
//use crate::palette;
use crate::render::{render, render_model_skin};

use pak::Pak;
use sdl2::{event::Event, keyboard::Keycode};

mod config;
mod models;
mod pak;
mod palette;
mod render;

fn main() -> Result<(), String> {
    // Load the .PAK file
    let pak = Pak::new("id1/PAK0.PAK").expect("Failed to open PAK file");
    let files = pak.read_directory().unwrap();

    // Load the Quake palette
    let palette_data = pak.find_file("gfx/palette.lmp").unwrap();
    let converted_palette = palette::convert_palette(&palette_data);

    // Load the player.mdl file
    let mdl_data = pak.find_file("progs/player.mdl").expect("Model not found");
    let mut reader = std::io::Cursor::new(&mdl_data);

    // Parse the model header
    let header = ModelHeader::from_reader(&mut reader).expect("Failed to parse model header");

    // Parse the skins
    let skins = models::parse_skins(&mut reader, &header).expect("Failed to parse skins");
    let skins_clone = skins.clone();

    // Create the model object
    let model = Model {
        header,
        skin_data: skins,
        skin_vertices: vec![],   // Placeholder: Load or parse as needed
        model_triangles: vec![], // Placeholder: Load or parse as needed
    };

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Quake", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");

    let mut event_pump = sdl_context.event_pump()?;

    // Main game/rendering loop
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

        render(&mut canvas, &converted_palette, &model);
        //render(&mut canvas, &converted_palette, &model, &skins);

        // Control frame rate (72 FPS)
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 72));
    }

    Ok(())
}
