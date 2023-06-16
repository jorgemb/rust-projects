use perfect_maze_generator as maze_generator;

fn main() {
    let maze = maze_generator::PerfectMaze::new(2, 3, Some(0));
    println!("{maze}");
}
