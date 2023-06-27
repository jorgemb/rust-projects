use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
use conway_life::{Environment, SimCell, Viewport};

fn main() {
    let steps: usize = 100;
    let sleep_time = time::Duration::from_millis(100);

    let mut environment = Environment::default();
    environment.set_living(&[SimCell::new(0, 0), SimCell::new(0, 1), SimCell::new(0, -1)]);
    let mut viewport = Viewport::new(-10, 10, 20, 20);


    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();
    for step in 0..steps {
        environment.fill_viewport(&mut viewport);

        stdout.queue(cursor::SavePosition).unwrap();
        // stdout.write_all(format!("Step: {}\n", step).as_bytes()).unwrap();
        stdout.write_all(viewport.to_string().as_bytes()).unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        environment.simulate();
        thread::sleep(sleep_time);
    }
    // for i in (1..30).rev() {
    //     stdout.queue(cursor::SavePosition).unwrap();
    //     stdout.write_all(format!("{}: FOOBAR ", i).as_bytes()).unwrap();
    //     stdout.queue(cursor::RestorePosition).unwrap();
    //     stdout.flush().unwrap();
    //     thread::sleep(time::Duration::from_millis(100));
    //
    //     stdout.queue(cursor::RestorePosition).unwrap();
    //     stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    // }
    stdout.execute(cursor::Show).unwrap();

    println!("Done!");
}