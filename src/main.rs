use std::{
    io::stdout,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::{Duration, Instant},
};
use crossterm::{
    cursor::Hide,
    event::{poll, read, Event, KeyCode},
    execute, terminal::enable_raw_mode,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Spans, Text},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

fn calculate_remaining_time(end_time: Instant) -> (Duration, u64, u64, u64, u32) {
    let remaining_time = end_time.duration_since(Instant::now());
    let hours = remaining_time.as_secs() / 3600;
    let remainder = remaining_time.as_secs() % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;
    let millis = remaining_time.subsec_millis();

    (remaining_time, hours, minutes, seconds, millis)
}

fn countdown(t: u64, round_count: Arc<Mutex<u64>>, reset: Arc<Mutex<bool>>) {
    let total_time = Duration::from_secs(t * 60);
    let end_time = Instant::now() + total_time;
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to initialize terminal");
    terminal.clear().expect("Failed to clear terminal");

    while Instant::now() < end_time {
        if *reset.lock().expect("Failed to acquire lock on reset") {
            *reset.lock().expect("Failed to acquire lock on reset") = false;
            *round_count.lock().expect("Failed to acquire lock on round_count") = 0;
            break;
        }
        let (remaining_time, hours, minutes, seconds, millis) = calculate_remaining_time(end_time);
        let elapsed_time = total_time - remaining_time;
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(90),
                        Constraint::Length(1),
                    ].as_ref()
                )
                .split(f.size());
            let block = Block::default()
                .title(Spans::from(format!(
                    " Countdown Timer | Round: {} ",
                    *round_count.lock().unwrap()
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White));
            let progress = elapsed_time.as_secs() as f64 / total_time.as_secs() as f64;
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Yellow))
                .percent((progress * 100.0) as u16);
            let paragraph = Paragraph::new(Text::from(format!(
                    "Time remaining: {:02}:{:02}:{:02}.{:03}",
                    hours, minutes, seconds, millis
                )))
                .block(block)
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center); 
            f.render_widget(paragraph, chunks[0]);
            f.render_widget(gauge, chunks[1]);
            })
            .expect("Failed to draw on terminal");
        sleep(Duration::from_millis(1));
    }
    terminal.clear().expect("Failed to clear terminal");
}

fn timer(round_count: Arc<Mutex<u64>>, reset: Arc<Mutex<bool>>) {
    loop {
        countdown(15, Arc::clone(&round_count), Arc::clone(&reset));
        *round_count.lock().expect("Failed to acquire lock on round_count") += 1;
    }
}

fn main() {
    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    let round_count = Arc::new(Mutex::new(1));
    let reset = Arc::new(Mutex::new(false));
    let round_count_clone = Arc::clone(&round_count);
    let reset_clone = Arc::clone(&reset);
    thread::spawn(move || {
        timer(round_count_clone, reset_clone);
    });

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                if key_event.code == KeyCode::Char('r') {
                    *round_count.lock().expect("Failed to acquire lock on round_count") = 1;
                    *reset.lock().expect("Failed to acquire lock on reset") = true;
                }
            }
        }
    }
}
