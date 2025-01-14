use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;

pub fn handle_music() {
    // Create an audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("id1/music/track06.ogg").unwrap());
    let source = Decoder::new(file).unwrap();
    // Play the music in a separate thread
    stream_handle
        .play_raw(source.convert_samples())
        .expect("Failed to play audio");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
