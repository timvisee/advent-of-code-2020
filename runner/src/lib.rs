#![feature(
    array_windows,
    custom_inner_attributes,
    iterator_fold_self,
    str_split_once
)]

macro_rules! fn_day {
    ($day:ident, $file:expr) => {{
        mod $day {
            include!($file);
        }
        $day::main
    }};
}

pub fn jobs() -> &'static [fn()] {
    &[
        fn_day!(day01a, "../../day01a/src/main.rs"),
        fn_day!(day01b, "../../day01b/src/main.rs"),
        fn_day!(day02a, "../../day02a/src/main.rs"),
        fn_day!(day02b, "../../day02b/src/main.rs"),
        fn_day!(day03a, "../../day03a/src/main.rs"),
        fn_day!(day03b, "../../day03b/src/main.rs"),
        fn_day!(day04a, "../../day04a/src/main.rs"),
        fn_day!(day04b, "../../day04b/src/main.rs"),
        fn_day!(day05a, "../../day05a/src/main.rs"),
        fn_day!(day05b, "../../day05b/src/main.rs"),
        fn_day!(day06a, "../../day06a/src/main.rs"),
        fn_day!(day06b, "../../day06b/src/main.rs"),
        fn_day!(day07a, "../../day07a/src/main.rs"),
        fn_day!(day07b, "../../day07b/src/main.rs"),
        fn_day!(day08a, "../../day08a/src/main.rs"),
        fn_day!(day08b, "../../day08b/src/main.rs"),
        fn_day!(day09a, "../../day09a/src/main.rs"),
        fn_day!(day09b, "../../day09b/src/main.rs"),
        fn_day!(day10a, "../../day10a/src/main.rs"),
        fn_day!(day10b, "../../day10b/src/main.rs"),
        fn_day!(day11a, "../../day11a/src/main.rs"),
        fn_day!(day11b, "../../day11b/src/main.rs"),
        fn_day!(day12a, "../../day12a/src/main.rs"),
        fn_day!(day12b, "../../day12b/src/main.rs"),
        fn_day!(day13a, "../../day13a/src/main.rs"),
        fn_day!(day13b, "../../day13b/src/main.rs"),
        fn_day!(day14a, "../../day14a/src/main.rs"),
        fn_day!(day14b, "../../day14b/src/main.rs"),
        fn_day!(day15a, "../../day15a/src/main.rs"),
        fn_day!(day15b, "../../day15b/src/main.rs"),
        fn_day!(day16a, "../../day16a/src/main.rs"),
        fn_day!(day16b, "../../day16b/src/main.rs"),
        fn_day!(day17a, "../../day17a/src/main.rs"),
        fn_day!(day17b, "../../day17b/src/main.rs"),
        fn_day!(day18a, "../../day18a/src/main.rs"),
        fn_day!(day18b, "../../day18b/src/main.rs"),
        // TODO: 19
        fn_day!(day20a, "../../day20a/src/main.rs"),
        fn_day!(day20b, "../../day20b/src/main.rs"),
    ]
}
