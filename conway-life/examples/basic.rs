use std::sync::mpsc;
use std::{io, thread};
use std::ops::Deref;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event as CEvent, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::{symbols, Terminal};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Widget};
use conway_life::{Environment, SimCell, Viewport};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("Couldn't enable raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("Poll not working") {
                if let CEvent::Key(key) = event::read().expect("Can read events") {
                    tx.send(Event::Input(key)).expect("Can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Setup the terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Setup life
    let mut conway_life = Environment::new();
    conway_life.set_living(&[SimCell::new(-1, 0), SimCell::new(0, 0), SimCell::new(1, 0)]);
    let mut viewport = Viewport::new(-10, 10, 20, 20);

    loop {
        terminal.draw(|rect| {
            let size = rect.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Min(5)
                    ]
                        .as_ref(),
                )
                .split(size);

            conway_life.fill_viewport(&mut viewport);
            let mut points = viewport.get_points();
            rect.render_widget(render_environment(&points), chunks[0]);
        })?;

        // Process input
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => conway_life.simulate() // Do a simulation step
        }
    }

    Ok(())
}

fn render_environment(points: &[(f64, f64)]) -> Chart {
    // Create dataset from viewport
    let datasets = vec![
        Dataset::default()
            // .name("data1")
            .marker(symbols::Marker::Block)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Cyan))
            .data(points),
    ];
    Chart::new(datasets)
        .block(Block::default().title("Conway's Game of Life").borders(Borders::ALL))
        .x_axis(Axis::default()
            // .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White)))
            // .labels(["-10.0", "0.0", "10.0"].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            // .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
            // .style(Style::default().fg(Color::White))
            .bounds([-10.0, 10.0]))
            // .labels(["-10.0", "0.0", "10.0"].iter().cloned().map(Span::from).collect()))
}

// fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//     let mut environment = Environment::default();
//     environment.set_living(&[SimCell::new(2, 1), SimCell::new(2, 0), SimCell::new(2, -1)]);
//
//     let mut viewport = Viewport::new(-10, 10, 20, 20);
//
//     loop {
//         terminal.draw(|frame| {
//             let size = frame.size();
//
//             let block = Block::default()
//                 .borders(Borders::ALL)
//                 .title("Conway's Game of Life")
//                 .title_alignment(Alignment::Center);
//             let inner_size = block.inner(size);
//             frame.render_widget(block, size);
//
//
//             // Check if viewport needs to be recreated
//             if viewport.width() != inner_size.width as usize || viewport.height() != inner_size.height as usize {
//                 let x = 0 - inner_size.width as i32 / 2;
//                 let y = inner_size.height as i32 / 2;
//                 viewport = Viewport::new(x, y, inner_size.width as usize, inner_size.height as usize);
//             }
//
//             environment.fill_viewport(&mut viewport);
//             let content = Paragraph::new(viewport.to_string());
//             frame.render_widget(content, inner_size);
//             environment.simulate();
//         })?;
//
//         if let Event::Key(key) = event::read()? {
//             if let KeyCode::Char('q') = key.code {
//                 return Ok(());
//             }
//         }
//     }
// }
