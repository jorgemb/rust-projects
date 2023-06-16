use std::io;
use std::str;

use perfect_maze_generator as maze_generator;

/// Asks for input from the user
fn ask_input<T>(prompt: &str) -> Option<T>
    where
        T: str::FromStr {
    // Show prompt and read next line
    println!("{prompt}");

    // Read line from user
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Error reading input");

    let value: Result<T, _> = line.trim().parse::<T>();
    match value {
        Ok(v) => Some(v),
        Err(_) => None
    }
}

fn main() {
    println!("Welcome to Perfect Maze Generator. Answer the following questions");
    println!("to generate a new maze.");

    // Ask the user for input
    let rows = ask_input::<usize>("Rows? ").unwrap();
    let columns = ask_input::<usize>("Columns? ").unwrap();

    let maze = maze_generator::PerfectMaze::new(columns, rows, None);
    println!("{maze}");
}
