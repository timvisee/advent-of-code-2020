# Advent of Code 2020 in Rust
My [Advent of Code 2020][aoc-2020] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

This year I attempt to develop a short, compact and fast solution for each
problem.

## Timings
Here is how long each solution takes to run to completion.
All solutions are measured (non scientifically) with [`hyperfine`][hyperfine] on
a `i5-4670k @ 3.8Ghz` machine running Linux.
Timings include binary execution, input reading and result printing delays.

|                                                | part A                          | part B                           |
|:-----------------------------------------------|:--------------------------------|:---------------------------------|
| [day 1](https://adventofcode.com/2020/day/1)   | [`0.4ms`](./day01a/src/main.rs) | [`0.3ms`](./day01b/src/main.rs)  |
| [day 2](https://adventofcode.com/2020/day/2)   | [`1.2ms`](./day02a/src/main.rs) |                                  |

## Run solutions
Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 1a, and run it
cd day01a
cargo run --release
```

Some solutions might require Rust Nightly.

## Previous years
- [2019](https://github.com/timvisee/advent-of-code-2019)
- [2018](https://github.com/timvisee/advent-of-code-2018)
- [2017](https://github.com/timvisee/advent-of-code-2017)

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[aoc-2020]: https://adventofcode.com/2020
[hyperfine]: https://github.com/sharkdp/hyperfine
