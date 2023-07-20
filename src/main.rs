use std::time::{Duration, Instant};
use std::thread::{self, sleep};
use std::io::stdout;
use std::sync::{Arc, Mutex};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use crossterm::execute;
use crossterm::cursor::Hide;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, Paragraph, Gauge};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Color, Style};
use tui::text::{Text, Spans};

fn countdown(t: u64, count: Arc<Mutex<u64>>, reset: Arc<Mutex<bool>>) {
    let total_time = Duration::from_secs(t * 60);
    let end_time = Instant::now() + total_time;
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
                        Constraint::Percentage(90),
                        Constraint::Length(1),
                    ].as_ref()
                )
                .split(f.size());
            let block = Block::default()
                .title(Spans::from(format!(
                    " Countdown Timer | Round: {} ",
                    *count.lock().unwrap()
                )))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White));
            let elapsed_time = total_time - remaining_time;
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
        }).unwrap();
        sleep(Duration::from_millis(1));  
    }
    terminal.clear().unwrap();
}

fn timer(count: Arc<Mutex<u64>>, reset: Arc<Mutex<bool>>) {
    loop {
        countdown(1, Arc::clone(&count), Arc::clone(&reset));
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
}
