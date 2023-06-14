use crate::*;

#[should_panic]
#[test]
fn invalid_maze() {
    // This should panic
    let maze = PerfectMaze::new(0, 0, None);
}

#[test]
fn default_maze() {
    let (columns, rows) = (10, 15);
    let seed = 0;
    let maze = PerfectMaze::new(columns, rows, Some(seed));

    // Check initial conditions
    assert_eq!(maze.columns(), columns);
    assert_eq!(maze.rows(), rows);
    assert_eq!(maze.seed(), seed);

    // Check wall status
    for row in 0..rows {
        for column in 0..columns {
            let right = maze.get_right_wall(row, column);
            assert!(right.is_some());

            let bottom = maze.get_bottom_wall(row, column);
            assert!(bottom.is_some());
        }
    }

    // Check walls that don't exist
    let not_right = maze.get_right_wall(rows, columns);
    assert!(not_right.is_none());

    let not_bottom = maze.get_bottom_wall(rows, columns);
    assert!(not_bottom.is_none());
}

#[test]
fn display_maze() {
    let expected =
        "_____
|_|_|
|_|_|
|_|_|
";

    let maze = PerfectMaze::new(2, 3, Some(0));

    assert_eq!(expected, maze.to_string());
}