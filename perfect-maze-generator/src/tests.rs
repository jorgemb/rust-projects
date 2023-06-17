use crate::*;

#[should_panic]
#[test]
fn invalid_maze() {
    // This should panic
    let _maze = PerfectMaze::new(0, 0, None);
}

#[test]
fn default_maze() {
    let (columns, rows) = (10, 15);
    let seed = 42;
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
    // Maze 2x3
    let expected =
        "_____
|   |
| | |
|_|_|
";

    let maze = PerfectMaze::new(2, 3, Some(0));
    assert_eq!(expected, maze.to_string());

    // Maze 1x1
    let expected = "___\n|_|\n";
    let maze = PerfectMaze::new(1, 1, None);
    assert_eq!(expected, maze.to_string());
}

#[test]
fn internal_values() {
    const COLUMNS: usize = 8;
    const ROWS: usize = 10;
    let maze = PerfectMaze::new(COLUMNS, ROWS, None);


    const WALLS_PER_ROW: usize = 2 * COLUMNS - 1;
    assert_eq!(maze.walls_per_row(), WALLS_PER_ROW);

    // Compare cell positions
    const C00: MazeCell = MazeCell { row: 0, column: 0, total_columns: COLUMNS };
    const C01: MazeCell = MazeCell { row: 0, column: 1, total_columns: COLUMNS };
    const C10: MazeCell = MazeCell { row: 1, column: 0, total_columns: COLUMNS };
    const C11: MazeCell = MazeCell { row: 1, column: 1, total_columns: COLUMNS };
    assert_eq!(maze.cell_pair_from_wall(0), (C00, C01));
    assert_eq!(maze.cell_pair_from_wall(COLUMNS - 1), (C00, C10));
    assert_eq!(maze.cell_pair_from_wall(WALLS_PER_ROW), (C10, C11));
    assert_eq!(maze.cell_pair_from_wall(COLUMNS), (C01, C11));
}