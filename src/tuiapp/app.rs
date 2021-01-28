use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Terminal,
};
use std::{
    error::Error,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
    io,
};

use argh::FromArgs;

enum Event<I> {
    Input(I),
    Tick,
}

/// Crossterm demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

pub struct TUIApp {
    should_quit: bool,

    cli: Cli,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TUIApp {
    pub fn new() -> TUIApp {
        let mut stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);

        let app = TUIApp {
            cli: argh::from_env(),
            terminal: Terminal::new(backend).unwrap(),

            should_quit: false,
        };
        app
    }
    
    pub fn main_loop(&mut self) {
        // Setup input handling
        let (tx, rx) = mpsc::channel();

        let tick_rate = Duration::from_millis(self.cli.tick_rate);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                // poll for tick rate duration, if no events, sent tick event.
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                if event::poll(timeout).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        tx.send(Event::Input(key)).unwrap();
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    tx.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        loop {
            match rx.recv().unwrap() {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                        break;
                    }
                    _ => {}
                },
                Event::Tick => {
                    println!("tick");
                }
            }
            if self.should_quit {
                break;
            }
        }
    }
}
