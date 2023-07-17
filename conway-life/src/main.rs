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
use conway_life::application::App;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::default();
    app.run()?;

    Ok(())
}
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     enable_raw_mode().expect("Couldn't enable raw mode");
//
//     let (tx, rx) = mpsc::channel();
//     let tick_rate = Duration::from_millis(200);
//     thread::spawn(move || {
//         let mut last_tick = Instant::now();
//         loop {
//             let timeout = tick_rate
//                 .checked_sub(last_tick.elapsed())
//                 .unwrap_or_else(|| Duration::from_secs(0));
//
//             if event::poll(timeout).expect("Poll not working") {
//                 if let CEvent::Key(key) = event::read().expect("Can read events") {
//                     tx.send(Event::Input(key)).expect("Can send events");
//                 }
//             }
//
//             if last_tick.elapsed() >= tick_rate {
//                 if let Ok(_) = tx.send(Event::Tick) {
//                     last_tick = Instant::now();
//                 }
//             }
//         }
//     });
//
//     // Setup the terminal
//     let stdout = io::stdout();
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     terminal.clear()?;
//
//     // Setup life
//     let mut conway_life = Environment::default();
//     conway_life.set_living(&[
//         SimCell::new(0, 1), SimCell::new(1, 1),
//         SimCell::new(-1, 0), SimCell::new(0, 0),
//         SimCell::new(0, -1)]
//     );
//     let mut viewport = Box::new(Viewport::new(-10, 10, 20, 20));
//     let mut simulation_time = Duration::from_millis(0);
//
//     loop {
//
//         terminal.draw(|rect| {
//             let size = rect.size();
//
//             let chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .margin(2)
//                 .constraints(
//                     [
//                         Constraint::Min(5)
//                     ]
//                         .as_ref(),
//                 )
//                 .split(size);
//
//             // Recalculate viewport if necessary
//             let inner_rect = chunks[0].inner(&Margin { vertical: 1, horizontal: 1 });
//             let viewport_resize = viewport.width() != inner_rect.width as usize || viewport.height() != inner_rect.height as usize;
//
//             if viewport_resize {
//                 viewport = Box::new(Viewport::new(-((inner_rect.width / 2) as i32),
//                                                   (inner_rect.height / 2) as i32,
//                                                   inner_rect.width as usize,
//                                                   inner_rect.height as usize));
//                 conway_life.fill_viewport(&mut viewport);
//             }
//
//             rect.render_widget(render_environment(&viewport, simulation_time.as_micros()), chunks[0]);
//         })?;
//
//         // Process input
//         match rx.recv()? {
//             Event::Input(event) => match event.code {
//                 KeyCode::Char('q') => {
//                     disable_raw_mode()?;
//                     terminal.show_cursor()?;
//                     break;
//                 }
//                 _ => {}
//             },
//             Event::Tick => {
//                 let start_instant = Instant::now();
//                 conway_life.simulate();
//                 simulation_time = start_instant.elapsed();
//                 conway_life.fill_viewport(&mut viewport);
//             } // Do a simulation step
//         }
//     }
//
//     Ok(())
// }
//
// fn render_environment(viewport: &Viewport, simulation_time: u128) -> Paragraph {
//     Paragraph::new(viewport.to_string())
//         .block(Block::default()
//             .title(format!("Conway's Game of Life: x={}, y={}, width={}, height={} - ({}µs)",
//                            viewport.x(), viewport.y(), viewport.width(), viewport.height(), simulation_time))
//             .title_alignment(Alignment::Center)
//             .borders(Borders::ALL))
// }
