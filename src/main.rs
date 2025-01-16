use std::time::{Duration, Instant};

use crate::config::*;
use crate::models::{Model, ModelHeader};
use crate::render::*;

use bsp::{Edge, Vertex};
use glam::Vec3;
use music::handle_music;
use pak::Pak;
use sdl2::{event::Event, keyboard::Keycode};

mod bsp;
mod config;
mod models;
mod music;
mod pak;
mod palette;
mod render;
mod wad;

fn main() -> Result<(), String> {
    // Load the .PAK file
    let pak0 = Pak::new("id1/PAK0.PAK").expect("Failed to open PAK0 file");
    let pak1 = Pak::new("id1/PAK1.PAK").expect("Failed to open PAK1 file");

    // Load the Quake palette
    let palette_data = pak0.find_file("gfx/palette.lmp").unwrap();
    let converted_palette = palette::convert_palette(&palette_data);

    // Load the player.mdl file
    let mdl_data = pak0.find_file("progs/player.mdl").expect("Model not found");
    let mut reader = std::io::Cursor::new(&mdl_data);

    // Parse the model header
    let header = ModelHeader::from_reader(&mut reader).expect("Failed to parse model header");

    // Parse the skins
    let skins = models::parse_skins(&mut reader, &header).expect("Failed to parse skins");

    // Create the model object
    let model = Model {
        header,
        skin_data: skins,
        skin_vertices: vec![],   // Placeholder: Load or parse as needed
        model_triangles: vec![], // Placeholder: Load or parse as needed
    };

    let wad = wad::Wad::new(pak0.find_file("gfx.wad").unwrap());
    let bsp = bsp::Bsp::new(pak0.find_file("maps/e1m1.bsp").unwrap());

    let bsp_header = bsp.read_header();
    let vertices = bsp.read_vertices(&bsp_header);
    let edges = bsp.read_edges(&bsp_header);
    //let entities = bsp.read_entities(&bsp_header);
    let faces = bsp.read_faces(&bsp_header);
    let planes = bsp.read_planes(&bsp_header);
    println!("{:?}", planes);

    // handle_music(); todo: find a way to play music while being able to move and render the map

    //println!("{:?}", vertices);
    // Initialize SDL2
    let mut sdl_context = sdl2::init()?;
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

    let mut camera = Camera {
        position: Vec3::new(538.0, 284.0, 28.0), // hardcoded for start.bsp, will be dependent on level later
        forward: Vec3::new(0.0, 1.0, 0.0),       // Looking toward Y
        up: Vec3::new(0.0, 0.0, 1.0),            // Z is up
        right: Vec3::new(1.0, 0.0, 0.0),         // X is right
        yaw: 0.0,
        pitch: 0.0,
        fov: 125.0,
        aspect_ratio: 320.0 / 200.0,
        near: 0.1,
        far: 1200.0,
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut last_frame_time = Instant::now();
    let move_speed = 310.0; // ranger max run speed

    // Main game/rendering loop
    'running: loop {
        //println!("yaw : {}", camera.yaw);
        let now = Instant::now();
        let delta_time = (now - last_frame_time).as_secs_f32();
        last_frame_time = now;
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
                _ => {
                    handle_input(
                        &event,
                        &mut camera,
                        delta_time,
                        move_speed,
                        &mut sdl_context,
                    );
                }
            }
        }

        render(
            &mut canvas,
            &converted_palette,
            &model,
            &camera,
            &vertices,
            &edges,
            &faces,
        );

        // Control frame rate (72 FPS)
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 72));
    }

    Ok(())
}
