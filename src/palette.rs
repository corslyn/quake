use std::io;

use byteorder::ReadBytesExt;

pub fn convert_palette(palette: Vec<u8>) -> Vec<(u8, u8, u8)> {
    let mut cursor = io::Cursor::new(&palette);
    let mut converted_palette = Vec::new();

    // Each color is represented by 3 bytes (red, green, blue)
    for _ in 0..(palette.len() / 3) {
        let red = cursor.read_u8().unwrap();
        let green = cursor.read_u8().unwrap();
        let blue = cursor.read_u8().unwrap();
        converted_palette.push((red, green, blue));
    }

    converted_palette
}
