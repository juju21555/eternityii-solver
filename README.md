# Eternity 2 solver

This code try to solve small instance of an Eternity II puzzle using a simulated annealing method in Rust and the X algorithm in C.

## Example of a 4x4 instance with 4 colors

```bash
------------------------------------
|   2   ||   0   ||   0   ||   1   |
| 2 - 1 || 3 - 3 || 3 - 0 || 2 - 3 |
|   2   ||   3   ||   1   ||   3   |
------------------------------------
------------------------------------
|   3   ||   0   ||   0   ||   2   |
| 3 - 2 || 3 - 1 || 1 - 0 || 2 - 2 |
|   0   ||   2   ||   1   ||   2   |
------------------------------------
------------------------------------
|   0   ||   1   ||   2   ||   0   |
| 1 - 0 || 0 - 2 || 1 - 1 || 1 - 1 |
|   1   ||   3   ||   0   ||   3   |
------------------------------------
------------------------------------
|   3   ||   3   ||   0   ||   1   |
| 2 - 2 || 2 - 2 || 0 - 0 || 3 - 0 |
|   0   ||   1   ||   0   ||   1   |
------------------------------------
```

Solution :

```bash
------------------------------------
|   1   ||   0   ||   0   ||   0   |
| 0 - 2 || 2 - 3 || 3 - 0 || 0 - 0 |
|   3   ||   3   ||   1   ||   0   |
------------------------------------
------------------------------------
|   3   ||   3   ||   1   ||   0   |
| 1 - 3 || 3 - 0 || 0 - 1 || 1 - 1 |
|   2   ||   3   ||   0   ||   2   |
------------------------------------
------------------------------------
|   2   ||   3   ||   0   ||   2   |
| 2 - 2 || 2 - 2 || 2 - 2 || 2 - 2 |
|   1   ||   1   ||   3   ||   2   |
------------------------------------
------------------------------------
|   1   ||   1   ||   3   ||   2   |
| 3 - 0 || 0 - 1 || 1 - 1 || 1 - 3 |
|   1   ||   0   ||   0   ||   0   |
------------------------------------
```

## Execute code

### Rust

Build : `cargo build --release`

Run : `cargo run --release`

Mean time for solving a 4x4 instances with 4 colors : 0.5s

### C

Build : `gcc c/algo_x.c`

Run : `./a.out` or `./a.exe`

Mean time for solving a 5x5 instances with 5 colors : 0.5s
