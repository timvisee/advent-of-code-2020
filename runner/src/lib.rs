#![feature(str_split_once)]

macro_rules! fn_day {
    ($day:ident, $file:expr) => {{
        mod $day {
            include!($file);
        }
        $day::main
    }};
}

pub fn jobs() -> Vec<fn()> {
    let mut jobs: Vec<fn()> = Vec::with_capacity(25 * 2);

    jobs.push(fn_day!(day01a, "../../day01a/src/main.rs"));
    jobs.push(fn_day!(day01b, "../../day01b/src/main.rs"));
    jobs.push(fn_day!(day02a, "../../day02a/src/main.rs"));
    jobs.push(fn_day!(day02b, "../../day02b/src/main.rs"));
    jobs.push(fn_day!(day03a, "../../day03a/src/main.rs"));
    jobs.push(fn_day!(day03b, "../../day03b/src/main.rs"));
    jobs.push(fn_day!(day04a, "../../day04a/src/main.rs"));
    jobs.push(fn_day!(day04b, "../../day04b/src/main.rs"));
    jobs.push(fn_day!(day05a, "../../day05a/src/main.rs"));
    jobs.push(fn_day!(day05b, "../../day05b/src/main.rs"));
    jobs.push(fn_day!(day06a, "../../day06a/src/main.rs"));
    jobs.push(fn_day!(day06b, "../../day06b/src/main.rs"));
    jobs.push(fn_day!(day07a, "../../day07a/src/main.rs"));
    jobs.push(fn_day!(day07b, "../../day07b/src/main.rs"));
    jobs.push(fn_day!(day08a, "../../day08a/src/main.rs"));
    jobs.push(fn_day!(day08b, "../../day08b/src/main.rs"));
    jobs.push(fn_day!(day09a, "../../day09a/src/main.rs"));
    jobs.push(fn_day!(day09b, "../../day09b/src/main.rs"));

    jobs
}