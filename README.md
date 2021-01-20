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

|                                                | part A                              | part B                              |
|:-----------------------------------------------|:------------------------------------|:------------------------------------|
| [day 1](https://adventofcode.com/2020/day/1)   | [`000.169ms`](./day01a/src/main.rs) | [`000.007ms`](./day01b/src/main.rs) |
| [day 2](https://adventofcode.com/2020/day/2)   | [`000.553ms`](./day02a/src/main.rs) | [`000.067ms`](./day02b/src/main.rs) |
| [day 3](https://adventofcode.com/2020/day/3)   | [`000.008ms`](./day03a/src/main.rs) | [`000.012ms`](./day03b/src/main.rs) |
| [day 4](https://adventofcode.com/2020/day/4)   | [`000.158ms`](./day04a/src/main.rs) | [`000.184ms`](./day04b/src/main.rs) |
| [day 5](https://adventofcode.com/2020/day/5)   | [`000.003ms`](./day05a/src/main.rs) | [`000.011ms`](./day05b/src/main.rs) |
| [day 6](https://adventofcode.com/2020/day/6)   | [`000.032ms`](./day06a/src/main.rs) | [`000.057ms`](./day06b/src/main.rs) |
| [day 7](https://adventofcode.com/2020/day/7)   | [`000.002ms`](./day07a/src/main.rs) | [`001.740ms`](./day07b/src/main.rs) |
| [day 8](https://adventofcode.com/2020/day/8)   | [`000.020ms`](./day08a/src/main.rs) | [`000.133ms`](./day08b/src/main.rs) |
| [day 9](https://adventofcode.com/2020/day/9)   | [`000.043ms`](./day09a/src/main.rs) | [`000.024ms`](./day09b/src/main.rs) |
| [day 10](https://adventofcode.com/2020/day/10) | [`000.004ms`](./day10a/src/main.rs) | [`000.004ms`](./day10b/src/main.rs) |
| [day 11](https://adventofcode.com/2020/day/11) | [`000.006ms`](./day11a/src/main.rs) | [`006.120ms`](./day11b/src/main.rs) |
| [day 12](https://adventofcode.com/2020/day/12) | [`000.014ms`](./day12a/src/main.rs) | [`000.012ms`](./day12b/src/main.rs) |
| [day 13](https://adventofcode.com/2020/day/13) | [`000.002ms`](./day13a/src/main.rs) | [`000.005ms`](./day13b/src/main.rs) |
| [day 14](https://adventofcode.com/2020/day/14) | [`000.282ms`](./day14a/src/main.rs) | [`005.870ms`](./day14b/src/main.rs) |
| [day 15](https://adventofcode.com/2020/day/15) | [`000.228ms`](./day15a/src/main.rs) | [`541.000ms`](./day15b/src/main.rs) |
| [day 16](https://adventofcode.com/2020/day/16) | [`000.226ms`](./day16a/src/main.rs) | [`000.557ms`](./day16b/src/main.rs) |
| [day 17](https://adventofcode.com/2020/day/17) | [`000.375ms`](./day17a/src/main.rs) | [`008.070ms`](./day17b/src/main.rs) |
| [day 18](https://adventofcode.com/2020/day/18) | [`000.224ms`](./day18a/src/main.rs) | [`000.232ms`](./day18b/src/main.rs) |
| [day 19](https://adventofcode.com/2020/day/19) | [`000.369ms`](./day19a/src/main.rs) | [`000.577ms`](./day19b/src/main.rs) |
| [day 20](https://adventofcode.com/2020/day/20) | [`000.117ms`](./day20a/src/main.rs) | [`000.462ms`](./day20b/src/main.rs) |
| [day 21](https://adventofcode.com/2020/day/21) | [`000.462ms`](./day21a/src/main.rs) | [`000.296ms`](./day21b/src/main.rs) |
| [day 22](https://adventofcode.com/2020/day/22) | [`000.002ms`](./day22a/src/main.rs) | [`003.430ms`](./day22b/src/main.rs) |
| [day 23](https://adventofcode.com/2020/day/23) | [`000.005ms`](./day23a/src/main.rs) | [`276.000ms`](./day23b/src/main.rs) |
| [day 24](https://adventofcode.com/2020/day/24) | [`000.108ms`](./day24a/src/main.rs) | [`042.300ms`](./day24b/src/main.rs) |
| [day 25](https://adventofcode.com/2020/day/25) | [`026.000ms`](./day25a/src/main.rs) |                                     |

|              | one-by-one                             | parallel                                   |
|:-------------|:---------------------------------------|:-------------------------------------------|
| _everything_ | [`917ms`](./runner/src/bin/runner.rs)  | [`657ms`](./runner/src/bin/runner-par.rs)  |

## Run solutions
Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# One solution requires large stack size, set to allow unlimited size
ulimit -s unlimited

# Switch to day 1a, and run it
cd day01a
cargo run --release

# Or run everything in parallel
cd ../runner
cargo run --release --bin runner-par

# Or benchmark every day
cd ../runner
cargo run --release --bin bench
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
