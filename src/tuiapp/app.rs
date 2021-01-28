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
    widgets::{Block, BorderType, Borders, ListState},
    Terminal,
};
use std::{
    error::Error,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
    io,
};

use crate::tuiapp::ui;

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

#[derive(Debug, Copy)]
pub struct TUIAppState {

}

impl Clone for TUIAppState {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct TUIApp {
    should_quit: bool,

    state: TUIAppState,

    cli: Cli,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TUIAppState {
    pub fn new() -> TUIAppState {
        TUIAppState {
        }
    }
}

impl TUIApp {
    pub fn new() -> TUIApp {
        let mut stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);

        let app = TUIApp {
            cli: argh::from_env(),
            terminal: Terminal::new(backend).unwrap(),

            state: TUIAppState::new(),

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

        self.terminal.clear();

        loop {
            let state = self.state;
            self.terminal.draw(|f| ui::draw(f, state));
            match rx.recv().unwrap() {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                        break;
                    }
                    _ => {}
                },
                Event::Tick => {
                    // println!("tick");
                }
            }
            if self.should_quit {
                break;
            }
        }
    }
}


pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
