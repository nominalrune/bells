use rodio::{source::Source, Decoder, OutputStream};
use std::{
    fs::File,
    time::Duration,
    thread::{sleep, spawn},
    io::{BufReader, stderr, Write},
};

fn main() {
    let seconds = 60*60+10;
    count_down(seconds, ||{play("bell.mp3");});
}

fn play(_file: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(_file).unwrap());
    let source = Decoder::new(file).unwrap();
    let res = stream_handle.play_raw(source.convert_samples());
    if res.is_err() {
        println!("Error playing sound: {:?}", res.err());
    } else {
        sleep(Duration::from_secs(12));
    }
}

fn count_down(seconds: i32, callback: fn()) {
    let interval = Duration::from_secs(1);
    let mut _seconds = seconds;
    let mut stream = stderr();
    loop {
        _seconds -= 1;
        if _seconds <= 0 {
            spawn(move || {
                callback();
            });
            _seconds = seconds;
        }
        stream
            .write(format!("\rnext: {:02}:{:02}:{:02}", _seconds / 3600, _seconds / 60 % 60, _seconds % 60).as_bytes())
            .unwrap();
        stream.flush().unwrap();
        sleep(interval);
    }
}
