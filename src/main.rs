use std::time::{Duration, Instant};
use std::thread::{self, sleep};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use crossterm::event::{poll, read, Event, KeyCode};

fn countdown(t: u64, count: Arc<Mutex<u64>>) {
    let end_time = Instant::now() + Duration::from_secs(t * 60);
    while Instant::now() < end_time {
        let remaining_time = end_time.duration_since(Instant::now());
        let hours = remaining_time.as_secs() / 3600;
        let remainder = remaining_time.as_secs() % 3600;
        let minutes = remainder / 60;
        let seconds = remainder % 60;
        let millis = remaining_time.subsec_millis();
        print!("\rTime remaining: {:02}:{:02}:{:02}.{:03} , T{}", hours, minutes, seconds, millis, *count.lock().unwrap());
        io::stdout().flush().unwrap(); // Flush the output buffer
        sleep(Duration::from_millis(1));  // sleep for 1 millisecond
    }
    print!("\rNext Round, T{}", *count.lock().unwrap());
    io::stdout().flush().unwrap(); // Flush the output buffer
}

fn timer(count: Arc<Mutex<u64>>) {
    loop {
        countdown(15, Arc::clone(&count));
        *count.lock().unwrap() += 1;
    }
}

fn main() {
    let count = Arc::new(Mutex::new(1));
    let count_clone = Arc::clone(&count);
    thread::spawn(move || {
        timer(count_clone);
    });

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                if key_event.code == KeyCode::Char('r') {
                    *count.lock().unwrap() = 1;
                }
            }
        }
    }
}
