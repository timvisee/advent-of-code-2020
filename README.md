# Advent of Code 2020 in Rust
My [Advent of Code 2020][aoc-2020] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

This year I attempt to develop a standalone, short, compact and fast solution
for each problem (day part).

## Timings
Here is how long each solution takes to run to completion.
All solutions are measured (non scientifically) with [`hyperfine`][hyperfine] on
a `i5-4670k @ 3.8Ghz` machine running Linux.
Timings include binary loading, execution, input and output timings.

|                                                | part A                          | part B                           |
|:-----------------------------------------------|:--------------------------------|:---------------------------------|
| [day 1](https://adventofcode.com/2020/day/1)   | [`0.4ms`](./day01a/src/main.rs) | [`0.3ms`](./day01b/src/main.rs)  |
| [day 2](https://adventofcode.com/2020/day/2)   | [`1.2ms`](./day02a/src/main.rs) | [`0.2ms`](./day02b/src/main.rs)  |
| [day 3](https://adventofcode.com/2020/day/3)   | [`0.4ms`](./day03a/src/main.rs) | [`0.4ms`](./day03b/src/main.rs)  |
| [day 4](https://adventofcode.com/2020/day/4)   | [`0.7ms`](./day04a/src/main.rs) | [`0.7ms`](./day04b/src/main.rs)  |
| [day 5](https://adventofcode.com/2020/day/5)   | [`0.3ms`](./day05a/src/main.rs) | [`0.3ms`](./day05b/src/main.rs)  |
| [day 6](https://adventofcode.com/2020/day/6)   | [`0.4ms`](./day06a/src/main.rs) | [`0.4ms`](./day06b/src/main.rs)  |
| [day 7](https://adventofcode.com/2020/day/7)   | [`3.0ms`](./day07a/src/main.rs) | [`2.5ms`](./day07b/src/main.rs)  |
| [day 8](https://adventofcode.com/2020/day/8)   | [`0.4ms`](./day08a/src/main.rs) | [`0.4ms`](./day08b/src/main.rs)  |
| [day 9](https://adventofcode.com/2020/day/9)   | [`0.4ms`](./day09a/src/main.rs) | [`0.3ms`](./day09b/src/main.rs)  |
| [day 10](https://adventofcode.com/2020/day/10) | [`0.3ms`](./day10a/src/main.rs) | [`0.3ms`](./day10b/src/main.rs)  |
| [day 11](https://adventofcode.com/2020/day/11) | [`5.3ms`](./day11a/src/main.rs) | [`6.4ms`](./day11b/src/main.rs)  |
| [day 12](https://adventofcode.com/2020/day/12) | [`0.2ms`](./day12a/src/main.rs) | [`0.3ms`](./day12b/src/main.rs)  |
| [day 13](https://adventofcode.com/2020/day/13) | [`0.3ms`](./day13a/src/main.rs) | [`0.2ms`](./day13b/src/main.rs)  |
| [day 14](https://adventofcode.com/2020/day/14) | [`0.6ms`](./day14a/src/main.rs) | [`6.4ms`](./day14b/src/main.rs)  |
| [day 15](https://adventofcode.com/2020/day/15) | [`0.1ms`](./day15a/src/main.rs) | [`528ms`](./day15b/src/main.rs)  |
| [day 16](https://adventofcode.com/2020/day/16) | [`0.7ms`](./day16a/src/main.rs) | [`1.0ms`](./day16b/src/main.rs)  |
| [day 17](https://adventofcode.com/2020/day/17) | [`0.5ms`](./day17a/src/main.rs) | [`8.3ms`](./day17b/src/main.rs)  |

|              | one-by-one                             | parallel                                   |
|:-------------|:---------------------------------------|:-------------------------------------------|
| _everything_ | [`488ms`](./runner/src/bin/runner.rs)  | [`462ms`](./runner/src/bin/runner-par.rs)  |

## Run solutions
Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 1a, and run it
cd day01a
cargo run --release

# Or run everything in parallel
cd ../runner
cargo run --release --bin runner-par
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
