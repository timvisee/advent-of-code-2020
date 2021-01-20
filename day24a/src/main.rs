#![feature(array_windows, split_inclusive)]

use std::collections::HashSet;

pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split_inclusive(|&b| b == b'\n')
            .map(|line| {
                line.array_windows().fold((0, 0), |(x, y), v| match v {
                    [b'e', _] => (x, y + 1),
                    [b'w', _] => (x, y - 1),
                    [b's', b'e'] => (x + 1, y),
                    [b's', b'w'] => (x + 1, y + 1),
                    [b'n', b'e'] => (x - 1, y - 1),
                    [b'n', b'w'] => (x - 1, y),
                    _ => unreachable!(),
                })
            })
            .fold(HashSet::new(), |mut map, coord| {
                if !map.remove(&coord) {
                    map.insert(coord);
                }
                map
            })
            .len()
    );
}
