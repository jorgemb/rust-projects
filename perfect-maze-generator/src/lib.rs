use std::fmt::{Display, Formatter, Write};
use rand::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct PerfectMaze {
    columns: usize,
    rows: usize,
    seed: u32,
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
    /// # Panic
    /// It will panic if width or height is 0
    pub fn new(columns: usize, rows: usize, seed: Option<u32>) -> Self {
        assert_ne!(columns, 0);
        assert_ne!(rows, 0);

        // Generate seed
        let mut generator = thread_rng();
        let seed = seed.unwrap_or_else(|| generator.next_u32());

        // Set walls (and fill with true)
        let total_walls = (columns - 1) * rows + (rows - 1) * columns;
        let walls = vec![true; total_walls];

        // Do not randomize walls if seed is zero
        if seed != 0 {
            // TODO
        }

        // Create
        PerfectMaze { columns, rows, seed, walls }
    }

    /// Returns the amount of walls in a row (both horizontal + vertical)
    #[inline]
    fn walls_per_row(&self) -> usize{
        2*self.columns - 1
    }

    /// Validates if the current cell is valid
    #[inline]
    fn is_valid_cell(&self, row: usize, column: usize) -> Option<()>{
        if row >= self.rows || column >= self.columns{
            return None;
        }

        Some(())
    }

    /// Returns the status of the right wall of the cell. If the cell is not valid then None
    /// is returned.
    fn get_right_wall(&self, row: usize, column: usize) -> Option<bool> {
        self.is_valid_cell(row, column)?;

        // If we are in the last column, the right wall is always up
        if column == self.columns-1{
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
        if row == self.rows-1{
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
    pub fn seed(&self) -> u32 {
        self.seed
    }
}
