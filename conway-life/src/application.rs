//! Contains the modules to show the user interface of the simulator.

use std::{io, thread};
use std::io::Stdout;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use thiserror::Error;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Margin};
use tui::Terminal;
use tui::widgets::{Block, Borders, Paragraph};

use crate::{SimCell, Viewport};

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Error with terminal application")]
    Terminal(#[from] io::Error),

    #[error("Error while transmitting information")]
    Channel(#[from] std::sync::mpsc::RecvError),
}

/// Represents an event happening within the application.
enum AppEvent {
    Input(KeyEvent),
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

        App { environment, viewport, show_stats }
    }
}

impl App {
    /// Starts the application loop
    pub fn run(&mut self) -> Result<(), ApplicationError> {
        let mut terminal = App::setup_terminal()?;
        let (tx, rx) = mpsc::channel();

        // Run the input thread
        let default_tick_rate = Duration::from_millis(200);
        let input_thread = thread::spawn(move || App::handle_input(default_tick_rate, tx));

        // Run the main loop
        loop {
            // Update

            // Draw
            terminal.draw(|rect| {
                let area = rect.size();

                // Resize viewport if necessary
                let target_area = area.inner(&Margin { horizontal: 1, vertical: 1 });
                if target_area.width as usize != self.viewport.width() || target_area.height as usize != self.viewport.height() {
                    let width = target_area.width as usize;
                    let height = target_area.height as usize;
                    let x = -((width / 2) as i32);
                    let y = (height / 2) as i32;

                    self.viewport = Viewport::new(x, y, width, height);
                }

                rect.render_widget(self.render_environment(), area);
            })?;

            // Handle input
            match rx.recv()? {
                AppEvent::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
                AppEvent::Quit => break,
                AppEvent::Tick => {
                    // let start_instant = Instant::now();
                    self.environment.simulate();
                    // simulation_time = start_instant.elapsed();
                    self.environment.fill_viewport(&mut self.viewport);
                }
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
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        Ok(terminal)
    }

    /// Clean's up the terminal for the following process
    fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), ApplicationError> {
        disable_raw_mode()?;
        terminal.show_cursor()?;

        Ok(())
    }

    /// Handle input and events
    fn handle_input(tick_rate: Duration, sender: Sender<AppEvent>) {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("Poll not working") {
                // Send the key events
                if let Event::Key(key) = event::read().expect("Can't read events") {
                    if sender.send(AppEvent::Input(key)).is_err() {
                        // Close the input thread on transmission error
                        break;
                    }

                    // Check if it is escape key
                    if key.code == KeyCode::Esc {
                        let _ = sender.send(AppEvent::Quit);
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

    /// Render the environment
    fn render_environment(&mut self) -> Paragraph {
        // Create title
        let title = if self.show_stats {
            format!("Conway's Game of Life: X={}, Y={}, W={}, H={}",
                    self.viewport.x(),
                    self.viewport.y(),
                    self.viewport.width(),
                    self.viewport.height())
        } else {
            String::from("Conway's Game of Life")
        };

        // Create paragraph
        Paragraph::new(self.viewport.to_string())
            .block(Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL))
    }
}