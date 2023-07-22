use std::{io, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event as CEvent, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Margin};
use tui::widgets::{Block, Borders, Paragraph};

use conway_life::{Environment, SimCell, Viewport};
use conway_life::application::{App, ApplicationError};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), ApplicationError> {
    let mut app = App::default();
    app.run()
}
