use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use rand::prelude::*;
use rand_xoshiro::Xoshiro256StarStar as RandomGenerator;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct PerfectMaze {
    columns: usize,
    rows: usize,
    seed: u64,
    walls: Vec<bool>,
}

impl Display for PerfectMaze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vertical_walls = self.columns - 1;
        let horizontal_walls = self.columns;

        // Maze characters
        const H_WALL: char = '_';
        const V_WALL: char = '|';
        const EMPTY: char = ' ';
        const NEWLINE: char = '\n';

        // Top row
        let top_wall_size = vertical_walls + horizontal_walls + 2;
        for _ in 0..top_wall_size {
            f.write_char(H_WALL)?;
        }
        f.write_char(NEWLINE)?;

        // Rows
        for row in 0..self.rows {
            f.write_char(V_WALL)?;

            for column in 0..self.columns {
                // Bottom wall
                if self.get_bottom_wall(row, column).unwrap() {
                    f.write_char(H_WALL)?;
                } else {
                    f.write_char(EMPTY)?;
                }

                // Right wall
                if self.get_right_wall(row, column).unwrap() {
                    f.write_char(V_WALL)?;
                } else {
                    f.write_char(EMPTY)?;
                }
            }
            f.write_char(NEWLINE)?;
        }


        Ok(())
    }
}

impl PerfectMaze {
    /// Creates a new MazeGenerator with the given dimensions.
    ///
    /// A perfect maze is one where for any two points there exist one, and
    /// only one path between them. An algorithm that follows a left or right
    /// wall should be able to deterministically visit every single point
    /// in the maze.
    ///
    /// * `columns`: Amount of columns (width) of the maze.
    /// * `rows`: Amount of rows (height) of the maze.
    /// * `seed`: Value to use when randomizing the maze. A value of `None`
    /// calculates a random seed, and `Some(0)` will prevent wall randomization.
    ///
    /// # Panic
    /// It will panic if `width` or `height` is 0.
    pub fn new(columns: usize, rows: usize, seed: Option<u64>) -> Self {
        assert_ne!(columns, 0);
        assert_ne!(rows, 0);

        // Generate seed
        let seed = seed.unwrap_or_else(|| {
            let mut generator = rand::thread_rng();
            generator.next_u64()
        });

        // Set walls (and fill with true)
        let total_walls = (columns - 1) * rows + (rows - 1) * columns;
        let walls = vec![true; total_walls];

        // Create the list of wall indices
        // Do not randomize walls if seed is zero
        let mut wall_indices: Vec<usize> = (0..total_walls).collect();
        if seed != 0 {
            let mut generator = RandomGenerator::seed_from_u64(seed);
            wall_indices.shuffle(&mut generator);
        }

        // Create
        let mut maze = PerfectMaze { columns, rows, seed, walls };
        maze.tumble_walls(&wall_indices);

        maze
    }


    /// Returns the amount of walls in a row (both horizontal + vertical)
    #[inline]
    fn walls_per_row(&self) -> usize {
        2 * self.columns - 1
    }

    /// Validates if the current cell is valid
    #[inline]
    fn is_valid_cell(&self, row: usize, column: usize) -> Option<()> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        Some(())
    }

    /// Returns the status of the right wall of the cell. If the cell is not valid then None
    /// is returned.
    fn get_right_wall(&self, row: usize, column: usize) -> Option<bool> {
        self.is_valid_cell(row, column)?;

        // If we are in the last column, the right wall is always up
        if column == self.columns - 1 {
            return Some(true);
        }

        // Find the wall id and return the status
        let wall_id = row * self.walls_per_row() + column;
        Some(self.walls[wall_id])
    }

    /// Returns the status of the bottom wall of the cell. If the cell is not valid then None
    /// is returned.
    fn get_bottom_wall(&self, row: usize, column: usize) -> Option<bool> {
        self.is_valid_cell(row, column)?;

        // If we are in the last row, the bottom wall is always up
        if row == self.rows - 1 {
            return Some(true);
        }

        // Find the wall id and return the status
        let wall_id = row * self.walls_per_row() + (self.columns - 1) + column;
        Some(self.walls[wall_id])
    }

    /// Returns the number of columns in the maze (a.k.a. width)
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the number of rows in the maze (a.k.a. height)
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the seed used to initialize the maze
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns the cell pair that is separated by the given wall
    fn cell_pair_from_wall(&self, wall_id: usize) -> (Cell, Cell) {
        let current_row = wall_id / self.walls_per_row();
        let wall_in_row = wall_id % self.walls_per_row();
        let is_vertical = wall_in_row < (self.columns() - 1);
        let total_columns = self.columns();

        if is_vertical {
            let cell_a = Cell { row: current_row, column: wall_in_row, total_columns };
            let cell_b = Cell { row: current_row, column: wall_in_row + 1, total_columns };
            (cell_a, cell_b)
        } else {
            let column = wall_in_row - (self.columns() - 1);
            let cell_a = Cell { row: current_row, column, total_columns };
            let cell_b = Cell { row: current_row + 1, column, total_columns };
            (cell_a, cell_b)
        }
    }

    /// Returns the set that contains the cell
    fn get_set_with_cell(cell_sets: &[HashSet<usize>], cell_id: usize) -> Option<usize> {
        cell_sets.iter().enumerate().find_map(|(set_id, set)| if set.contains(&cell_id) {
            Some(set_id)
        } else {
            None
        })
    }

    /// Applies the wall tumbling algorithm to the list of walls
    fn tumble_walls(&mut self, wall_indices: &[usize]) {
        // Initialize sets
        let total_cells = self.rows() * self.columns();
        let mut cell_sets = Vec::with_capacity(total_cells);
        for index in 0..total_cells {
            let set = HashSet::from([index; 1]);
            cell_sets.push(set);
        }

        // Iterate through the wall indices
        for current_wall in wall_indices {
            let (cell_a, cell_b) = self.cell_pair_from_wall(*current_wall);

            // Search the set of each cell
            let id_set_a = Self::get_set_with_cell(&cell_sets, cell_a.id()).unwrap();
            let id_set_b = Self::get_set_with_cell(&cell_sets, cell_b.id()).unwrap();

            if id_set_a != id_set_b {
                // Wall can be tumbled
                self.walls[*current_wall] = false;

                // Merge sets
                let set_a = cell_sets.get(id_set_a).unwrap();
                let set_b = cell_sets.get(id_set_b).unwrap();
                let new_set: HashSet<_> = set_a.union(set_b).cloned().collect();

                cell_sets[id_set_a] = new_set;
                cell_sets[id_set_b] = HashSet::new();
            }
        }
    }
}

/// Represents a cell within the Maze.
#[derive(Debug)]
struct Cell {
    row: usize,
    column: usize,
    total_columns: usize,
}

impl Cell {
    /// Returns the ID of the cell within the maze
    fn id(&self) -> usize { self.row * self.total_columns + self.column }
}