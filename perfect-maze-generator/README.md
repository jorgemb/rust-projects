# Perfect Maze Generator

Generate a 2D orthogonal perfect maze in the CLI. A perfect maze is one where for any
two points A and B only a single path exists that connects them. This means that the 
maze can be traversed following a single wall.

``` shell
cargo run -p perfect-maze-generator --rows 10 --columns 10 --seed 128
```

Example output
``` text
_____________________
|_     _| |  _|  _| |
|_ _| |    _ _ _| | |
|    _|_| |  _  | | |
|_|   |     |_ _|_  |
|_ _| | |_| | | | | |
|   |_  | | |    _ _|
|_|  _  |  _| |_ _ _|
|  _|_ _| |_     _  |
|_|_ _     _ _|_|  _|
|_ _ _ _|_ _ _ _|_ _|
```

## Algorithm


## TODO

- [ ] Allow maze definitions to be saved and loaded from files
- [ ] Traverse the maze from point A to point B
- [ ] Open holes in two places from the edges for entry and exit
- [ ] Finish internal documentation