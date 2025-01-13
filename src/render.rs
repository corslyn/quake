use sdl2::{pixels::Color, render::WindowCanvas, sys::Window};

use crate::palette::*;

pub fn render(canvas: &mut WindowCanvas, palette: &[(u8, u8, u8)]) {
    // Clear the canvas with a black background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Render the palette
    render_palette(canvas, palette);

    // Present the canvas to display the rendered content
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
