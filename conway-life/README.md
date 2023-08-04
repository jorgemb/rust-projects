# Conway's Game of Life
This is a basic simulation of Conway's Game of Life, which should support an infinite (or as large as `i32` variables go)
environment. The basic example shows the F-Pentomimmo in simulation.

![F-Pentomimo](https://pi.math.cornell.edu/~lipa/mec/f.png)

You can run the example using:
```bash
cargo run -p conway-life
```

Withing the environment it possible to interact with the environment using text commands like:

```
stats | t -> Shows/Hides statistics
coord | c -> Shows/Hides current coordinates
pause | p -> Pause/Unpause the simulation
quit | q -> Quit the simulation
load <path> | l <path> -> Load an environment (in YAML) in the given route
save <path> | s <path> -> Save the current environment into the given route
```

## TODO

- ~~Allow to save and load environments from files~~
- Add interaction in the CLI
  - Pan across the environment
  - ~~Play / Pause~~
  - Increase / Decrease simulation velocity
  - ~~Show / Hide the number of iteration, number of living cells or other statistics~~
- Add an editor to create and manage environments with
  - Load and save buttons
  - Set / Unset living
  - Mouse support
- Benchmark to see that it is efficient