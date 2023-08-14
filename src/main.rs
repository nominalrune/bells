use rodio::{source::Source, Decoder, OutputStream};
use std::{
    fs::File,
    time::Duration,
    cmp::Ordering,
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
    let mut _hours = seconds / 3600;
    let mut _minutes = (seconds % 3600) / 60;
    let mut _seconds = seconds % 60;
    let mut stream = stderr();
    loop {
        match _seconds.cmp(&0) {
            Ordering::Less => {
                _minutes -= 1;
                _seconds = 59;
                if _hours > 0 && _minutes <= 0 {
                    _hours -= 1;
                    _minutes = 59;
                }
            }
            Ordering::Equal => {
                _minutes -= 1;
                _seconds = 59;
                if _hours > 0 && _minutes <= 0 {
                    _hours -= 1;
                    _minutes = 59;
                }
            }
            Ordering::Greater => {
                _seconds -= 1;
            }
        }
        if _hours == 0 && _minutes == 0 && _seconds == 0 {
            spawn(move || {
                callback();
            });
            _hours = seconds / 3600;
            _minutes = seconds / 60;
            _seconds = seconds % 60;
        }
        stream
            .write(format!("\r{:02}:{:02}:{:02}", _hours, _minutes, _seconds).as_bytes())
            .unwrap();
        stream.flush().unwrap();
        sleep(interval);
    }
}
