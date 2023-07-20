use std::time::{Duration, Instant};
use std::thread::{self, sleep};
use std::io::{self, Write, stdout};
use std::sync::{Arc, Mutex};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::execute;
use crossterm::cursor::Hide;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Color, Modifier, Style};
use tui::text::{Text, Spans};

fn countdown(t: u64, count: Arc<Mutex<u64>>, reset: Arc<Mutex<bool>>) {
    let end_time = Instant::now() + Duration::from_secs(t * 60);
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    while Instant::now() < end_time {
        if *reset.lock().unwrap() {
            *reset.lock().unwrap() = false;
            *count.lock().unwrap() = 0;
            break;
        }
        let remaining_time = end_time.duration_since(Instant::now());
        let hours = remaining_time.as_secs() / 3600;
        let remainder = remaining_time.as_secs() % 3600;
        let minutes = remainder / 60;
        let seconds = remainder % 60;
        let millis = remaining_time.subsec_millis();
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(100),
                    ].as_ref()
                )
                .split(f.size());
            let block = Block::default()
                .title(Spans::from(format!(
                    "Countdown Timer | Round: {}",
                    *count.lock().unwrap()
                )))
                .title_style(Style::default().fg(Color::Yellow).bg(Color::Black).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White));
            let paragraph = Paragraph::new(Text::from(format!(
                    "Time remaining: {:02}:{:02}:{:02}.{:03}",
                    hours, minutes, seconds, millis
                )))
                .block(block)
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center); // Set alignment to Center
            f.render_widget(paragraph, chunks[0]);
        }).unwrap();
        sleep(Duration::from_millis(1));  // sleep for 1 millisecond
    }
    terminal.clear().unwrap();
}

fn timer(count: Arc<Mutex<u64>>, reset: Arc<Mutex<bool>>) {
    loop {
        countdown(15, Arc::clone(&count), Arc::clone(&reset));
        *count.lock().unwrap() += 1;
    }
}

fn main() {
    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
    let count = Arc::new(Mutex::new(1));
    let reset = Arc::new(Mutex::new(false));
    let count_clone = Arc::clone(&count);
    let reset_clone = Arc::clone(&reset);
    thread::spawn(move || {
        timer(count_clone, reset_clone);
    });

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                if key_event.code == KeyCode::Char('r') {
                    *count.lock().unwrap() = 1;
                    *reset.lock().unwrap() = true;
                }
            }
        }
    }
    disable_raw_mode().unwrap();
}
