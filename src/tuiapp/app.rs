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
    fs,
};
use std::path::{ Path, PathBuf };

use crate::tuiapp::ui;

use crate::launcher_config;
use crate::download::version_list::{ MinecraftVersionListJson, MinecraftVersionJson };
use crate::download::version::MinecraftVersionInfoJson;

use crate::tuiapp::{ Focus };

use crate::loader::{ MinecraftVersion, MinecraftInstance };

use argh::FromArgs;

enum Event<I> {
    Input(I),
    Tick,
}

enum StateEvent {
    State(TUIAppState),
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

#[derive(Debug)]
pub struct TUIAppState {
    pub focused: Focus,

    pub selected_version_id_in_all_list: String,
    pub selected_version_id_in_installed_list: String,

    pub manifest_items: Option<StatefulList<MinecraftVersionJson>>,
    pub installed_items: Option<StatefulList<MinecraftVersion>>,

    pub instance: MinecraftInstance,
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
            focused: Focus::INSTALLED_VERSION_LIST,
            selected_version_id_in_all_list: String::new(),
            selected_version_id_in_installed_list: String::new(),

            manifest_items: None,
            installed_items: None,

            instance: MinecraftInstance::new(),
        }
    }

    pub fn create_with_mc_path(path: &Path) -> TUIAppState {
        if !path.exists() || !path.is_dir() {
            return TUIAppState {
                focused: Focus::INSTALLED_VERSION_LIST,
                selected_version_id_in_all_list: String::new(),
                selected_version_id_in_installed_list: String::new(),
    
                manifest_items: None,
                installed_items: None,

                instance: MinecraftInstance::from(path.to_str().unwrap_or_else(|| { "" })),
            }
        }

        let mut instance = MinecraftInstance::from(path.to_str().unwrap_or_else(|| { "" }));

        // Load installed versions
        let mut installed_versions = instance.existing_versions();

        let versions_folder_path: PathBuf = path.join("versions");
        let mut manifest_file_path = versions_folder_path;
        manifest_file_path.push("version_manifest_v2.json");

        if !manifest_file_path.exists() || !manifest_file_path.is_file() {
            return TUIAppState {
                focused: Focus::INSTALLED_VERSION_LIST,
                selected_version_id_in_all_list: String::new(),
                selected_version_id_in_installed_list: String::new(),
    
                manifest_items: None,
                installed_items: Some(StatefulList::with_items(installed_versions)),

                instance: instance,
            }
        }

        let contents = fs::read_to_string(manifest_file_path)
            .expect("Something went wrong reading the file");
        let version_list: crate::download::version_list::MinecraftVersionListJson
            = serde_json::from_str(&contents).unwrap();

        TUIAppState {
            focused: Focus::INSTALLED_VERSION_LIST,
            selected_version_id_in_all_list: String::new(),
            selected_version_id_in_installed_list: String::new(),

            manifest_items: Some(StatefulList::with_items(version_list.versions)),
            installed_items: Some(StatefulList::with_items(installed_versions)),

            instance: instance,
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

            state: TUIAppState::create_with_mc_path(Path::new("/Users/inoki/Library/Application Support/minecraft")),
            // state: TUIAppState::new(),

            should_quit: false,
        };
        app
    }
    
    pub fn main_loop(&mut self) -> Option<MinecraftVersionInfoJson> {
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
            {
                let state = &mut self.state;
                self.terminal.draw(|f| ui::draw(f, state));
            }
            match rx.recv().unwrap() {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                        break;
                    }
                    KeyCode::Char('r') => {
                        match self.state.focused {
                            Focus::ALL_VERSION_LIST => {
                                // TODO: use fully async way
                                match self.state.instance.download_manifest() {
                                    Some(manifest) => {
                                        self.state.manifest_items = Some(StatefulList::with_items(manifest.versions));
                                    }
                                    None => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char('s') => {
                        // Start
                        match self.state.focused {
                            Focus::INSTALLED_VERSION_LIST => {
                                // Do something to bring the game up
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char('c') => {
                        // Cancel download
                        match self.state.focused {
                            Focus::DOWNLOAD_PAGE => {
                                self.state.focused = Focus::ALL_VERSION_LIST;
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char('d') => {
                        // Download
                        self.state.focused = Focus::DOWNLOAD_PAGE;
                    }
                    KeyCode::Char('p') => {
                        // Print launch command
                    }
                    KeyCode::Tab => {
                        // Change focused panel
                        match self.state.focused {
                            Focus::INSTALLED_VERSION_LIST => {
                                self.state.focused = Focus::ALL_VERSION_LIST;
                            }
                            Focus::ALL_VERSION_LIST => {
                                // Currently not focus on status list
                                self.state.focused = Focus::INSTALLED_VERSION_LIST;
                                // self.state.focused = Focus::STATUS_LIST;
                            }
                            Focus::STATUS_LIST => {
                                self.state.focused = Focus::INSTALLED_VERSION_LIST;
                            }
                            _ => {
                                self.state.focused = Focus::ALL_VERSION_LIST;
                            }
                        }
                    }
                    KeyCode::Up => {
                        // Change focus up
                        match self.state.focused {
                            Focus::INSTALLED_VERSION_LIST => {
                                match &mut self.state.installed_items {
                                    Some(list) => {
                                        list.previous();
                                    }
                                    None => {}
                                }
                            }
                            Focus::ALL_VERSION_LIST => {
                                match &mut self.state.manifest_items {
                                    Some(list) => {
                                        list.previous();
                                    }
                                    None => {}
                                }
                            }
                            Focus::STATUS_LIST => {
                            }
                            _ => {
                            }
                        }
                    }
                    KeyCode::Down => {
                        // Change focus down
                        match self.state.focused {
                            Focus::INSTALLED_VERSION_LIST => {
                                match &mut self.state.installed_items {
                                    Some(list) => {
                                        list.next();
                                    }
                                    None => {}
                                }
                            }
                            Focus::ALL_VERSION_LIST => {
                                match &mut self.state.manifest_items {
                                    Some(list) => {
                                        list.next();
                                    }
                                    None => {}
                                }
                            }
                            Focus::STATUS_LIST => {
                            }
                            _ => {
                            }
                        }
                    }
                    _ => {}
                },
                Event::Tick => {
                    // println!("tick");
                },
                _ => {
                },
            }
            if self.should_quit {
                break;
            }
        }
        None
    }
}


#[derive(Debug)]
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
