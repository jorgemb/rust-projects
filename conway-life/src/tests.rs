use crate::*;

#[test]
fn test_cell() {
    let (x, y) = (10, -22);

    // Basic validations
    let cell1 = SimCell { x, y };
    assert_eq!(x, cell1.x);
    assert_eq!(y, cell1.y);

    assert_eq!(cell1.to_string(), format!("({x},{y})"));

    // Comparison to other cells
    let cell2 = SimCell::new(x, y);
    assert_eq!(cell1, cell2);

    let cell3 = SimCell::new(0, 0);
    assert_ne!(cell1, cell3);
}

#[test]
fn empty_environment() {
    let mut env = Environment::new();

    // At start the environment is empty
    assert!(env.living_cells.is_empty());

    // After a simulation step the environment should still be empty
    env.simulate();
    assert!(env.living_cells.is_empty());
    assert!(!env.get_cell(&SimCell::new(0, 0)));
}

#[test]
fn change_environment() {
    let mut env = Environment::new();

    // Toggle single cell
    let cell = SimCell::new(0, 0);
    assert!(!env.get_cell(&cell), "Initial state is false");
    assert!(env.toggle_cell(&cell), "Toggling should make set to living");
    assert!(env.get_cell(&cell), "After toggling cell is alive");
    assert!(!env.toggle_cell(&cell), "Toggling again should set to dead");
    assert!(!env.get_cell(&cell), "Cell should be dead after toggling again");

    // Extend environment
    let living = vec![SimCell::new(-1, 0), SimCell::new(0, 0), SimCell::new(1, 0)];
    env.set_living(&living);
    living.iter().map(|c| assert!(env.get_cell(c))).count();
}

/// Checks if the environment contains only the given cells
fn check_environment(start_cells: &[SimCell], expected_cells: &[SimCell]) {
    // Initialize environment
    let mut env = Environment::new();
    env.set_living(start_cells);
    env.simulate();

    // Check all the cells
    expected_cells.iter().map(|c| assert!(env.get_cell(c))).count();

    // Check the size
    assert_eq!(env.living_cells.len(), expected_cells.len());
}

#[test]
fn simulate_still_lives() {
    // Block
    //  ----
    //  -xx-
    //  0xx-
    //  ----
    let block = [
        SimCell::new(1, 0), SimCell::new(1, 1),
        SimCell::new(2, 0), SimCell::new(2, 1)];
    check_environment(&block, &block);

    // Bee hive
    //  ------
    //  --xx--
    //  0x--x-
    //  --xx--
    //  ------
    let beehive = [
        SimCell::new(2, 1), SimCell::new(3, 1),
        SimCell::new(1, 0), SimCell::new(4, 0),
        SimCell::new(2, -1), SimCell::new(3, -1)];
    check_environment(&beehive, &beehive);

    // Tub
    // -----
    // --x--
    // 0x-x-
    // --x--
    // -----
    let tub = [
        SimCell::new(2, 1),
        SimCell::new(1, 0), SimCell::new(3, 0),
        SimCell::new(2, -1),
    ];
    check_environment(&tub, &tub);
}

#[test]
fn simulate_blinker() {
    // Start
    // -----
    // --x--
    // 0-x--
    // --x--
    // -----
    let start = [
        SimCell::new(2, 1),
        SimCell::new(2, 0),
        SimCell::new(2, -1)
    ];

    // End
    // -----
    // -----
    // 0xxx-
    // -----
    // -----
    let end = [
        SimCell::new(1, 0), SimCell::new(2, 0), SimCell::new(3, 0)
    ];

    check_environment(&start, &end);
    check_environment(&end, &start);
}

#[test]
fn simulate_toad() {
    // Start
    // ------
    // ------
    // --xxx-
    // 0xxx--
    // ------
    // ------
    let start = [
        SimCell::new(2, 1), SimCell::new(3, 1), SimCell::new(4, 1),
        SimCell::new(1, 0), SimCell::new(2, 0), SimCell::new(3, 0)
    ];

    // End
    // ------
    // ---x--
    // -x--x-
    // 0x--x-
    // --x---
    // ------
    let end = [
        SimCell::new(3, 2),
        SimCell::new(1, 1), SimCell::new(4, 1),
        SimCell::new(1, 0), SimCell::new(4, 0),
        SimCell::new(2, -1)
    ];

    check_environment(&start, &end);
    check_environment(&end, &start);
}

// Viewport
mod viewport_panics {
    use crate::Viewport;

    #[test]
    #[should_panic(expected = "width cannot be 0")]
    fn width() {
        Viewport::new(0, 0, 0, 1);
    }

    #[test]
    #[should_panic(expected = "height cannot be 0")]
    fn height() {
        Viewport::new(0, 0, 1, 0);
    }

    #[test]
    #[should_panic(expected = "width results in overflow")]
    fn width_overflow() {
        Viewport::new(0, 0, usize::MAX, 1);
    }

    #[test]
    #[should_panic(expected = "height results in overflow")]
    fn height_overflow() {
        Viewport::new(0, 0, 10, usize::MAX);
    }

    #[test]
    #[should_panic(expected = "width * height results in overflow")]
    fn size() {
        Viewport::new(i32::MIN, 0, usize::MAX / 2, 3);
    }
}

#[test]
fn viewport_basic() {
    let (x, y, width, height) = (-23, 44, 1024, 10500);
    let mut viewport = Viewport::new(x, y, width, height);

    assert_eq!(viewport.x(), x);
    assert_eq!(viewport.y(), y);
    assert_eq!(viewport.width(), width);
    assert_eq!(viewport.height(), height);
    assert_eq!(viewport.right(), x.wrapping_add_unsigned(width as u32));
    assert_eq!(viewport.bottom(), y.wrapping_sub_unsigned(height as u32));

    // Check belonging
    assert!(viewport.in_viewport(0, 0), "Origin should be in viewport");
    assert!(viewport.in_viewport(x, y), "Viewport origin should be in viewport");
    assert!(!viewport.in_viewport(x.wrapping_add_unsigned(width as u32), y), "Right should not be in viewport");
    assert!(!viewport.in_viewport(x, y.wrapping_add_unsigned(height as u32)), "Bottom should not be in viewport");

    // Check clearing
    viewport.data.iter().map(|&d| assert!(!d)).count();
    viewport.set_living(0, 0);
    viewport.clear();
    viewport.data.iter().map(|&d| assert!(!d)).count();
}

#[test]
fn viewport_display() {
    let mut viewport = Viewport::new(-1, 1, 3, 3);
    viewport.set_living(0, 0);

    let expected_repr = "   \n x \n   ";
    assert_eq!(expected_repr, viewport.to_string());
}

#[test]
fn environment_viewport() {
    let mut env = Environment::new();
    env.set_living(&[SimCell::new(0, 1), SimCell::new(-1, 0), SimCell::new(0, 0), SimCell::new(1, 0), SimCell::new(0, -1)]);

    let mut viewport = Viewport::new(-1, 1, 3, 3);
    env.fill_viewport(&mut viewport);

    let expected_repr = " x \nxxx\n x ";
    assert_eq!(expected_repr, viewport.to_string());
}