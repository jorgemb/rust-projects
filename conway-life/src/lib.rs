use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter, Write};

#[cfg(test)]
mod tests;

/// Represents a single cell within the simulation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct SimCell {
    pub x: i32,
    pub y: i32,
}

impl Display for SimCell {
    /// Displays the SimCell in `(x,y)` format.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl SimCell {
    /// Method to easily create new SimCells
    pub fn new(x: i32, y: i32) -> Self {
        SimCell { x, y }
    }
}

/// Represents an Environment that follows Conway's Game of Life rules. These are:
/// 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// 2. Any live cell with two or three live neighbours lives on to the next generation.
/// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
/// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
pub struct Environment {
    living_cells: BTreeSet<SimCell>,
}

impl Environment {
    /// Creates a new empty environment
    pub fn new() -> Self {
        Environment { living_cells: BTreeSet::new() }
    }

    /// Returns true if the given cell is alive
    pub fn get_cell(&self, cell: &SimCell) -> bool {
        self.living_cells.contains(cell)
    }

    /// Toggles a cell between living and dead.
    /// Returns the new value of the cell.
    pub fn toggle_cell(&mut self, cell: &SimCell) -> bool {
        if self.get_cell(cell) {
            // Set cell to dead
            self.living_cells.remove(cell);
            false
        } else {
            // Set cell to living
            self.living_cells.insert(*cell);
            true
        }
    }

    /// Sets a range to living
    pub fn set_living(&mut self, cells: &[SimCell]) {
        self.living_cells.extend(cells.iter())
    }

    /// Performs a simulation step, following the rules for the environment
    pub fn simulate(&mut self) {
        // Count how the neighborhood is affected
        let mut neighboors = HashMap::with_capacity(self.living_cells.len() * 9);
        for cell in self.living_cells.iter() {
            for x in (cell.x - 1)..=(cell.x + 1) {
                for y in (cell.y - 1)..=(cell.y + 1) {
                    // Create neighboring cell
                    let n = SimCell::new(x, y);
                    if n == *cell { continue; }

                    // Add to the neighbor
                    let count = neighboors.entry(n).or_insert(0u32);
                    *count += 1;
                }
            }
        }

        // Add new cells
        for new_living in neighboors.iter().filter(|(_, &v)| v == 3).map(|(c, _)| *c) {
            self.living_cells.insert(new_living);
        }

        // Remove any cell with less than 2 neighbors or more than 3
        self.living_cells
            .retain(|c|
                if let Some(&count) = neighboors.get(c) {
                    count == 2 || count == 3
                } else {
                    false
                });
    }

    /// Fills in a Viewport with the information from the simulation
    pub fn fill_viewport(&self, viewport: &mut Viewport) {
        viewport.clear();

        self.living_cells.iter().map(|c|
            if viewport.in_viewport(c.x, c.y) {
                viewport.set_living(c.x, c.y);
            }
        ).count();
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a viewport of an environment at a given position.
pub struct Viewport {
    x: i32,
    width: usize,
    y: i32,
    height: usize,
    data: Vec<bool>,
}

impl Viewport {
    /// Creates a new Viewport object.
    ///
    /// # Panics
    /// Will panic if any of the following conditions happen
    /// * `width` == 0
    /// * `height` == 0
    /// * `x + width` > i32_MAX
    /// * `y - height` < i32_MIN
    /// * `width * height` > usize_MAX
    pub fn new(x: i32, y: i32, width: usize, height: usize) -> Self {
        // Check preconditions
        assert_ne!(width, 0, "width cannot be 0");
        assert_ne!(height, 0, "height cannot be 0");

        let (_, overflowing_x) = x.overflowing_add_unsigned(width as u32);
        assert!(!overflowing_x, "X + width results in overflow");
        let (_, overflowing_y) = y.overflowing_sub_unsigned(height as u32);
        assert!(!overflowing_y, "y + height results in overflow");

        let (_, overflowing_size) = width.overflowing_mul(height);
        assert!(!overflowing_size, "width * height results in overflow");

        // Create the viewport vector
        let data = vec![false; width * height];
        Viewport { x, width, y, height, data }
    }

    /// Returns a vector with all the living points within the Viewport
    pub fn get_points<T: From<i32>>(&self) -> Vec<(T, T)> {
        let mut points = Vec::new();

        for (index, value) in self.data.iter().enumerate().filter(|&v| *v.1) {
            let x = T::from((index % self.width) as i32 + self.x);
            let y = T::from((index / self.width) as i32 - self.y);
            points.push((x, y));
        }

        points
    }

    /// Clears the whole buffer, setting every cell as dead
    pub fn clear(&mut self) {
        self.data.fill(false);
    }

    /// Returns if the given position is within the viewport
    #[inline]
    fn in_viewport(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.right() && y <= self.y && y > self.bottom()
    }

    /// Sets a position within the viewport as living
    pub fn set_living(&mut self, x: i32, y: i32) {
        assert!(self.in_viewport(x, y));

        let row = (x - self.x).abs() as usize;
        let column = (y - self.y).abs() as usize;
        let index = row * self.width + column;

        if let Some(c) = self.data.get_mut(index) {
            *c = true;
        }
    }


    /// Returns the left boundary of the Viewport (x)
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Returns the width of the Viewport
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the right boundary of the Viewport (x + width)
    pub fn right(&self) -> i32 {
        let (right, _) = self.x.overflowing_add_unsigned(self.width as u32);
        right
    }

    /// Returns the upper boundary of the Viewport (y)
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Returns the height of the Viewport
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the lower boundary of the Viewport (y + height)
    pub fn bottom(&self) -> i32 {
        let (bottom, _) = self.y.overflowing_sub_unsigned(self.height as u32);
        bottom
    }
}

impl Display for Viewport {
    /// A simple text based display of the Viewport
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const LIVING: char = 'x';
        const DEAD: char = ' ';

        for (i, val) in self.data.iter().enumerate() {
            // Check if newline is needed
            if i != 0 && i % self.width == 0 {
                f.write_char('\n')?;
            }
            if *val {
                f.write_char(LIVING)?;
            } else {
                f.write_char(DEAD)?;
            }
        }

        Ok(())
    }
}