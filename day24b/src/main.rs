#![feature(array_windows, split_inclusive)]

use std::collections::HashSet;

const NEIGHBORS: [(i16, i16); 6] = [(0, 1), (0, -1), (1, 1), (1, 0), (-1, 0), (-1, -1)];

fn main() {
    let map: HashSet<(i16, i16)> = include_bytes!("../input.txt")
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
        });

    println!("{}", (0..100).fold(map, |map, _| cycle(map)).len());
}

#[inline(always)]
#[rustfmt::skip]
fn cycle(old: HashSet<(i16, i16)>) -> HashSet<(i16, i16)> {
    let mut new = HashSet::new();
    for (x, y) in &old {
        // If 1 or 2 black neighbors, stay black
        if NEIGHBORS
            .iter()
            .map(|(xx, yy)| (x + xx, y + yy))
            .filter(|(x, y)| {
                // For each white neighbor, if 2 black neighbors, turn black
                let black = old.contains(&(*x, *y));
                if !black && NEIGHBORS
                        .iter()
                        .map(|(xx, yy)| (x + xx, y + yy))
                        .filter(|coord| old.contains(coord))
                        .take(3)
                        .count() == 2 {
                    new.insert((*x, *y));
                }
                black
            }).count() - 1 & !1 == 0 {
            new.insert((*x, *y));
        }
    }
    new
}
