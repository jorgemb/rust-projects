//! Contains the modules to show the user interface of the simulator.

use std::{fs, io, thread};
use std::io::{Read, Stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Paragraph};
use thiserror::Error;

use crate::{Environment, SimCell, Viewport};

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Error with terminal application")]
    Terminal(#[from] io::Error),

    #[error("Error while transmitting information")]
    Channel(#[from] std::sync::mpsc::RecvError),
}

/// Represents an event happening within the application.
enum AppEvent {
    ShowStats,
    ShowCoordinates,
    PartialInput(String),
    ErrorInput(String, String),
    Load(fs::File),
    Save(fs::File),
    Pause,
    Tick,
    Quit,
}

/// Main application object that manages the interaction and drawing
pub struct App {
    // Conway's Game of life specific
    environment: crate::Environment,
    viewport: crate::Viewport,

    // Application specific
    show_stats: bool,
    show_coordinates: bool,
    pause: bool,
    last_simulation_time: Duration,
    generation: usize,
    tick_time: Duration,
}

impl Default for App {
    /// Creates a default implementation App
    fn default() -> Self {
        // Setup environment and viewport
        let mut environment = crate::Environment::default();

        // Create the F-Pentomino
        environment.set_living(&[
            SimCell::new(0, 1), SimCell::new(1, 1),
            SimCell::new(-1, 0), SimCell::new(0, 0),
            SimCell::new(0, -1)]
        );

        let viewport = crate::Viewport::new(-10, 10, 20, 20);

        let show_stats = true;
        let show_coordinates = false;
        let last_simulation_time = Duration::from_secs(0);
        let tick_time = Duration::from_millis(50);
        let pause = false;
        let generation = 0;

        App { environment, viewport, show_stats, show_coordinates, pause, generation, last_simulation_time, tick_time }
    }
}

impl App {
    /// Starts the application loop
    pub fn run(&mut self) -> Result<(), ApplicationError> {
        let mut terminal = App::setup_terminal()?;
        let (tx, rx) = mpsc::channel();

        // Run the input thread
        let initial_tick_time = self.tick_time;
        let input_thread = thread::spawn(move || App::handle_input(initial_tick_time, tx));
        let mut current_input = String::default();
        let mut current_message = String::default();

        // Run the main loop
        loop {
            // Draw
            terminal.draw(|rect| {
                let area = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(4),
                        Constraint::Length(4)
                    ].as_ref())
                    .split(area);

                // SIMULATION VIEWPORT
                // Resize viewport if necessary
                let target_area = chunks[0];
                if target_area.width as usize != self.viewport.width() || target_area.height as usize != self.viewport.height() {
                    let width = target_area.width as usize;
                    let height = target_area.height as usize;
                    let x = -((width / 2) as i32);
                    let y = (height / 2) as i32;

                    self.viewport = Viewport::new(x, y, width, height);
                }

                rect.render_widget(self.render_environment(), target_area);


                // INPUT VIEWPORT
                let input_block = Paragraph::new(format!("{}\n{}", current_input, current_message))
                    .block(Block::default()
                        .title("Input")
                        .borders(Borders::ALL));
                rect.render_widget(input_block, chunks[1]);
            })?;

            // Handle input
            match rx.recv()? {
                AppEvent::Quit => break,
                AppEvent::Tick => {
                    if !self.pause {
                        let start_instant = Instant::now();
                        self.environment.simulate();
                        self.generation += 1;
                        self.last_simulation_time = start_instant.elapsed();
                    } else {
                        self.last_simulation_time = Duration::from_millis(0);
                    }

                    self.environment.fill_viewport(&mut self.viewport);
                }
                AppEvent::PartialInput(input) => {
                    current_input = input;
                    current_message.clear();
                }
                AppEvent::ErrorInput(input, message) => {
                    current_input = input;
                    current_message = message;
                }
                AppEvent::Load(mut file) => {
                    // Try loading the file
                    let mut environment_data = String::new();
                    let _ = file.read_to_string(&mut environment_data);
                    let loaded_env = serde_yaml::from_str::<Environment>(&environment_data);
                    if let Ok(loaded_env) = loaded_env {
                        self.environment = loaded_env;
                        self.generation = 0;
                        current_message = String::from("Loaded state from file");
                    }
                }
                AppEvent::Save(mut file) => {
                    let environment_data = serde_yaml::to_string(&self.environment);
                    if let Ok(environment_data) = environment_data {
                        let result = file.write_all(environment_data.as_bytes());
                        match result {
                            Ok(_) => current_message = String::from("Written state to file"),
                            Err(err) => current_message = format!("Unable to write state to file. Error: {}", err)
                        }
                    } else {}
                }
                AppEvent::ShowStats => self.show_stats = !self.show_stats,
                AppEvent::ShowCoordinates => self.show_coordinates = !self.show_coordinates,
                AppEvent::Pause => self.pause = !self.pause,
            }
        }

        App::cleanup_terminal(&mut terminal)?;
        drop(rx);
        input_thread.join().expect("Error closing input");

        Ok(())
    }

    /// Set's up the terminal so it is ready to be written by the UI
    fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, ApplicationError> {
        // Setup the terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        Ok(terminal)
    }

    /// Clean's up the terminal for the following process
    fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), ApplicationError> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    /// Handle input and events
    fn handle_input(tick_rate: Duration, sender: Sender<AppEvent>) {
        let mut last_tick = Instant::now();
        let mut current_input = String::default();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("Poll not working") {
                // Send the key events
                if let Event::Key(key) = event::read().expect("Can't read events") {
                    let result = match (key.code, key.kind) {
                        (KeyCode::Esc, KeyEventKind::Press) => sender.send(AppEvent::Quit),
                        // (KeyCode::Char('c'), KeyEventKind::Press) => sender.send(AppEvent::ShowCoordinates),
                        // (KeyCode::Char('s'), KeyEventKind::Press) => sender.send(AppEvent::ShowStats),
                        // (KeyCode::Char(' '), KeyEventKind::Press) => sender.send(AppEvent::Pause),
                        (KeyCode::Char(c), KeyEventKind::Press) => {
                            current_input.push(c);
                            sender.send(AppEvent::PartialInput(current_input.clone()))
                        }
                        (KeyCode::Backspace, KeyEventKind::Press) => {
                            current_input.pop();
                            sender.send(AppEvent::PartialInput(current_input.clone()))
                        }
                        (KeyCode::Enter, KeyEventKind::Press) => {
                            if !current_input.is_empty() {
                                let message = App::parse_input(&current_input);
                                current_input.clear();
                                sender.send(message)
                            } else {
                                // Ignore enter
                                sender.send(AppEvent::PartialInput(String::default()))
                            }
                        }
                        _ => Ok(())
                    };

                    // Break on an error
                    if result.is_err() {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = sender.send(AppEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    }

    /// Parses current input and returns a message to send
    fn parse_input(input: &str) -> AppEvent {
        let mut chunks = input.split(' ');

        if let Some(instruction) = chunks.next() {
            match instruction {
                "stats" | "t" => AppEvent::ShowStats,
                "coord" | "c" => AppEvent::ShowCoordinates,
                "pause" | "p" => AppEvent::Pause,
                "quit" | "q" => AppEvent::Quit,
                "load" | "l" => {
                    if let Some(path) = chunks.next() {
                        let file = fs::File::open(path);
                        if let Ok(file) = file {
                            AppEvent::Load(file)
                        } else {
                            AppEvent::ErrorInput(input.to_string(), String::from("File not found"))
                        }
                    } else {
                        AppEvent::ErrorInput(input.to_string(), String::from("File not specified"))
                    }
                }
                "save" | "s" => {
                    if let Some(path) = chunks.next() {
                        let file = fs::File::create(path);
                        if let Ok(file) = file {
                            AppEvent::Save(file)
                        } else {
                            AppEvent::ErrorInput(input.to_string(), format!("Unable to create file: {}", path))
                        }
                    } else {
                        AppEvent::ErrorInput(input.to_string(), String::from("File not specified"))
                    }
                }
                _ => AppEvent::ErrorInput(input.to_string(), String::from("Unknown instruction"))
            }
        } else {
            AppEvent::ErrorInput(input.to_string(), String::from("Invalid instruction"))
        }
    }

    /// Render the environment
    fn render_environment(&mut self) -> Paragraph {
        // Create title
        let coordinates = if self.show_coordinates {
            format!(" -- X={}, Y={}, W={}, H={}",
                    self.viewport.x(),
                    self.viewport.y(),
                    self.viewport.width(),
                    self.viewport.height())
        } else {
            String::from("")
        };

        let stats = if self.show_stats {
            format!(" -- Time={}µm, Living={}", self.last_simulation_time.as_micros(), self.environment.get_living_count())
        } else {
            String::default()
        };

        let title = format!("Conway's Game of Life -- GEN={}{}{}",
                            self.generation, coordinates, stats);

        // Create paragraph
        Paragraph::new(self.viewport.to_string())
            .block(Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL))
    }
}