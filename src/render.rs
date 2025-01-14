use sdl2::{pixels::Color, render::WindowCanvas};

use crate::models::*;
use crate::WIN_HEIGHT;
use crate::WIN_WIDTH;

pub fn render(canvas: &mut WindowCanvas, palette: &[(u8, u8, u8)], model: &Model) {
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

    // Stretch the off-screen texture to fit the current window size
    canvas
        .copy(&offscreen_texture, None, None)
        .expect("Failed to copy off-screen texture to canvas");

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
