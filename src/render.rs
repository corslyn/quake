pub use camera::Camera;
use glam::Vec3;
use glam::Vec4Swizzles;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{pixels::Color, render::WindowCanvas};

use crate::bsp::Edge;
use crate::bsp::Vertex;
use crate::models::*;
use crate::WIN_HEIGHT;
use crate::WIN_WIDTH;

mod camera;

pub fn render(
    canvas: &mut WindowCanvas,
    palette: &[(u8, u8, u8)],
    model: &Model,
    camera: &Camera,
    vertices: &Vec<Vertex>,
    edges: &[Edge],
) {
    // Create an off-screen texture at the fixed resolution (320x200)
    let texture_creator = canvas.texture_creator();
    let mut offscreen_texture = texture_creator
        .create_texture_target(sdl2::pixels::PixelFormatEnum::RGB24, WIN_WIDTH, WIN_HEIGHT)
        .expect("Failed to create off-screen texture");

    // Set the off-screen texture as the render target
    canvas
        .with_texture_canvas(&mut offscreen_texture, |texture_canvas| {
            // Clear the texture canvas with a black background
            texture_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            texture_canvas.clear();

            // Render the model skin at its native resolution (no scaling here)
            render_model_skin(texture_canvas, model, palette, &model.skin_data);
        })
        .expect("Failed to render to off-screen texture");

    // Reset the render target to the main canvas
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    let (window_width, window_height) = canvas.window().size();

    /*
        // Calculate the aspect ratio of the off-screen texture
        let texture_aspect_ratio = WIN_WIDTH as f32 / WIN_HEIGHT as f32;

        // Get the window dimensions
        let (window_width, window_height) = canvas.window().size();
        let window_aspect_ratio = window_width as f32 / window_height as f32;

        // Calculate the destination rectangle while maintaining the aspect ratio
        let (dest_width, dest_height) = if window_aspect_ratio > texture_aspect_ratio {
            // Window is wider than the texture; scale based on height
            let height = window_height;
            let width = (height as f32 * texture_aspect_ratio) as u32;
            (width, height)
        } else {
            // Window is taller than the texture; scale based on width
            let width = window_width;
            let height = (width as f32 / texture_aspect_ratio) as u32;
            (width, height)
        };

        let dest_x = ((window_width - dest_width) / 2) as i32;
        let dest_y = ((window_height - dest_height) / 2) as i32;

        let dest_rect = sdl2::rect::Rect::new(dest_x, dest_y, dest_width, dest_height);

        // Stretch the off-screen texture to fit the calculated rectangle
        canvas
            .copy(&offscreen_texture, None, Some(dest_rect))
            .expect("Failed to copy off-screen texture to canvas");
    */

    render_edges(canvas, camera, vertices, edges, window_width, window_height);

    // Present the canvas to display the final output
    canvas.present();
}

pub fn render_palette(canvas: &mut WindowCanvas, palette: &[(u8, u8, u8)]) {
    // Define the grid size and square dimensions
    let grid_width: usize = 16; // Number of colors per row
    let window_width = canvas.window().size().0;
    let window_height = canvas.window().size().1;

    let square_width = window_width / grid_width as u32;
    let square_height = window_height / (palette.len() as u32 / grid_width as u32);

    // Iterate through the palette and draw each color
    for (index, &(r, g, b)) in palette.iter().enumerate() {
        // Compute the row and column for this color
        let row = index / grid_width;
        let col = index % grid_width;

        // Calculate the position of the square
        let x = col as u32 * square_width;
        let y = row as u32 * square_height;

        // Set the draw color to the palette color
        canvas.set_draw_color(Color::RGB(r, g, b));

        // Draw the filled rectangle
        let rect = sdl2::rect::Rect::new(
            x as i32,
            y as i32,
            square_width,  // Subtract spacing
            square_height, // Subtract spacing
        );

        canvas.fill_rect(rect).expect("Failed to draw rectangle");
    }
}

pub fn render_model_skin(
    canvas: &mut WindowCanvas,
    model: &Model,
    palette: &[(u8, u8, u8)],
    skin_data: &[u8],
) {
    let skin_width = model.header.skinwidth as usize;
    let skin_height = model.header.skinheight as usize;

    // Create a texture for the skin
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            skin_width as u32,
            skin_height as u32,
        )
        .expect("Failed to create texture");

    // Map indexed skin data to RGB using the palette
    texture
        .with_lock(None, |buffer: &mut [u8], _| {
            for y in 0..skin_height {
                for x in 0..skin_width {
                    let index = skin_data[y * skin_width + x] as usize;
                    let color = palette[index];
                    let offset = (y * skin_width + x) * 3;
                    buffer[offset] = color.0; // Red
                    buffer[offset + 1] = color.1; // Green
                    buffer[offset + 2] = color.2; // Blue
                }
            }
        })
        .expect("Failed to update texture");

    // Render the texture at its original resolution
    let target_rect = sdl2::rect::Rect::new(0, 0, skin_width as u32, skin_height as u32);
    canvas
        .copy(&texture, None, Some(target_rect))
        .expect("Failed to copy texture to canvas");
}

fn transform_vertex(
    vertex: Vec3,
    camera: &Camera,
    window_width: u32,
    window_height: u32,
) -> Option<(i32, i32)> {
    let view_proj = camera.projection_matrix() * camera.view_matrix();
    let clip_space = view_proj * vertex.extend(1.0);

    if clip_space.w == 0.0 {
        return None; // Avoid division by zero
    }

    let ndc = clip_space.xyz() / clip_space.w; // Normalize device coordinates

    if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 || ndc.z < -1.0 || ndc.z > 1.0 {
        return None; // Outside clip space
    }

    // Map to screen space
    let x = ((ndc.x + 1.0) * 0.5 * window_width as f32) as i32;
    let y = ((1.0 - ndc.y) * 0.5 * window_height as f32) as i32;

    Some((x, y))
}

pub fn render_edges(
    canvas: &mut WindowCanvas,
    camera: &Camera,
    vertices: &[Vertex],
    edges: &[Edge],
    window_width: u32,
    window_height: u32,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for edge in edges {
        if let (Some(start), Some(end)) = (
            transform_vertex(
                vertices[edge.start_vertex as usize].coordinates,
                camera,
                window_width,
                window_height,
            ),
            transform_vertex(
                vertices[edge.end_vertex as usize].coordinates,
                camera,
                window_width,
                window_height,
            ),
        ) {
            canvas.set_draw_color(Color::RGB(255, 255, 255)); // Set edge color
            canvas.draw_line(start, end).expect("Failed to draw line");
        }
    }

    canvas.present();
}

pub fn handle_input(event: &Event, camera: &mut Camera, delta_time: f32, move_speed: f32) {
    match event {
        Event::KeyDown {
            keycode: Some(key), ..
        } => match key {
            &Keycode::W => camera.position += camera.forward * move_speed * delta_time,
            &Keycode::S => camera.position -= camera.forward * move_speed * delta_time,
            &Keycode::A => camera.position -= camera.right * move_speed * delta_time,
            &Keycode::D => camera.position += camera.right * move_speed * delta_time,
            &Keycode::Space => camera.position += camera.up * move_speed * delta_time,
            &Keycode::RCtrl => camera.position -= camera.up * move_speed * delta_time,
            &Keycode::UP => {
                camera.pitch += 5.0;
                camera.pitch = camera.pitch.clamp(-89.0, 89.0);
                camera.update_direction();
            }
            &Keycode::DOWN => {
                camera.pitch -= 5.0;
                camera.pitch = camera.pitch.clamp(-89.0, 89.0);
                camera.update_direction();
            }
            &Keycode::RIGHT => {
                camera.yaw += 5.0;
                camera.update_direction();
            }
            &Keycode::LEFT => {
                camera.yaw -= 5.0;
                camera.update_direction();
            }
            _ => {}
        },

        _ => {}
    }
}
