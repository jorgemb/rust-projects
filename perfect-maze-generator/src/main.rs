use clap::Parser;

use perfect_maze_generator as maze_generator;

/// Perfect Maze Generator can generate a random perfect maze, in which for any two points
/// only one path exists.
#[derive(Parser, Debug)]
struct Cli {
    /// Amount of rows to use. Cannot be 0.
    #[arg(long, short)]
    rows: usize,

    /// Amount of columns to use. Cannot be 0.
    #[arg(long, short)]
    columns: usize,

    /// Seed for randomizing the maze. A seed of 0 means no randomization is done.
    #[arg(long, short, default_value=None)]
    seed: Option<u64>,
}

fn main() {
    // Get CLI arguments
    let args = Cli::parse();

    let maze = maze_generator::PerfectMaze::new(args.columns, args.rows, args.seed);
    println!("{maze}");
}
