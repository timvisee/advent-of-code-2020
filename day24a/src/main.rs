#![feature(array_windows, split_inclusive)]

use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split_inclusive(|&b| b == b'\n')
            .map(|line| {
                line.array_windows()
                    .fold((0i32, 0i32), |(x, y), v| match v {
                        [b'e', _] => (x, y + 1),
                        [b'w', _] => (x, y - 1),
                        [b's', b'e'] => (x + 1, y),
                        [b's', b'w'] => (x + 1, y + 1),
                        [b'n', b'e'] => (x - 1, y - 1),
                        [b'n', b'w'] => (x - 1, y),
                        _ => unreachable!(),
                    })
            })
            .fold(HashMap::<_, bool>::new(), |mut map, coord| {
                map.entry(coord).and_modify(|b| *b = !*b).or_insert(true);
                map
            })
            .values()
            .filter(|&flipped| *flipped)
            .count()
    );
}
