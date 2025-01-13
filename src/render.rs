use sdl2::render::WindowCanvas;

use crate::palette::*;

pub fn render(canvas: &mut WindowCanvas) {
    self::render_palette(canvas);
    canvas.clear();
    canvas.present();
}

pub fn render_palette(canvas: &mut WindowCanvas) {}
