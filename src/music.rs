use rodio::Sink;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;

pub fn handle_music() {
    thread::spawn(|| {
        // Initialize Rodio Output Stream
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Load and play music
        let file = File::open("id1/music/track06.ogg").expect("Failed to open music file");
        let source = Decoder::new(BufReader::new(file))
            .expect("Failed to decode music file")
            .repeat_infinite();

        sink.append(source);
        sink.set_volume(1.0); // Full volume
        sink.sleep_until_end();
    });
}
