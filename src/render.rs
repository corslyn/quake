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
    canvas: &mut sdl2::render::WindowCanvas,
    camera: &Camera,
    vertices: &[Vertex],
    edges: &[Edge],
    screen_width: u32,
    screen_height: u32,
) {
    let view_proj = camera.projection_matrix() * camera.view_matrix();

    for edge in edges {
        let v1_world = vertices[edge.start_vertex as usize].coordinates;
        let v2_world = vertices[edge.end_vertex as usize].coordinates;

        let v1_clip = view_proj * v1_world.extend(1.0);
        let v2_clip = view_proj * v2_world.extend(1.0);

        if v1_clip.w <= 0.0 || v2_clip.w <= 0.0 {
            continue; // Skip edges behind the camera
        }

        let v1_ndc = v1_clip.xyz() / v1_clip.w;
        let v2_ndc = v2_clip.xyz() / v2_clip.w;

        if let Some((v1_clipped, v2_clipped)) = clip_edge_to_screen(v1_ndc, v2_ndc) {
            let v1_screen = ndc_to_screen(v1_clipped, screen_width, screen_height);
            let v2_screen = ndc_to_screen(v2_clipped, screen_width, screen_height);

            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            canvas
                .draw_line(v1_screen, v2_screen)
                .expect("Failed to draw edge");
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

fn clip_edge_to_screen(v1_ndc: Vec3, v2_ndc: Vec3) -> Option<(Vec3, Vec3)> {
    let mut v1 = v1_ndc;
    let mut v2 = v2_ndc;

    // Define NDC bounds
    let min_ndc = Vec3::new(-1.0, -1.0, -1.0);
    let max_ndc = Vec3::new(1.0, 1.0, 1.0);

    // Clip the edge to the NDC bounds
    let mut t_min: f32 = 0.0;
    let mut t_max: f32 = 1.0;

    for i in 0..3 {
        let delta = v2[i] - v1[i];

        if delta == 0.0 {
            // Parallel to this axis
            if v1[i] < min_ndc[i] || v1[i] > max_ndc[i] {
                return None; // Outside bounds
            }
        } else {
            // Calculate intersections with the planes
            let t1 = (min_ndc[i] - v1[i]) / delta;
            let t2 = (max_ndc[i] - v1[i]) / delta;

            // Order t1 and t2
            let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            t_min = t_min.max(t_near);
            t_max = t_max.min(t_far);

            if t_min > t_max {
                return None; // Fully outside
            }
        }
    }

    // Compute the clipped coordinates
    let v1_clipped = v1 + t_min * (v2 - v1);
    let v2_clipped = v1 + t_max * (v2 - v1);

    Some((v1_clipped, v2_clipped))
}

fn ndc_to_screen(ndc: Vec3, screen_width: u32, screen_height: u32) -> sdl2::rect::Point {
    let x = ((ndc.x + 1.0) * 0.5 * screen_width as f32) as i32;
    let y = ((1.0 - ndc.y) * 0.5 * screen_height as f32) as i32; // Flip Y axis
    sdl2::rect::Point::new(x, y)
}
