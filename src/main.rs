use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io::{self, Write};

fn countdown(t: u64, count: u64) {
    let end_time = Instant::now() + Duration::from_secs(t * 60);
    while Instant::now() < end_time {
        let remaining_time = end_time.duration_since(Instant::now());
        let hours = remaining_time.as_secs() / 3600;
        let remainder = remaining_time.as_secs() % 3600;
        let minutes = remainder / 60;
        let seconds = remainder % 60;
        let millis = remaining_time.subsec_millis();
        print!("\rTime remaining: {:02}:{:02}:{:02}.{:03} , T{}", hours, minutes, seconds, millis, count);
        io::stdout().flush().unwrap(); // Flush the output buffer
        sleep(Duration::from_millis(1));  // sleep for 1 millisecond
    }
    print!("\rNext Round, T{}", count);
    io::stdout().flush().unwrap(); // Flush the output buffer
}

fn timer() {
    let mut count = 1;
    loop {
        countdown(15, count);
        count += 1;
    }
}

fn main() {
    timer();
}

