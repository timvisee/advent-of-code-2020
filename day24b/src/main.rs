#![feature(array_windows, split_inclusive)]

use std::collections::{HashMap, HashSet};

const NEIGHBORS: [(i16, i16); 6] = [(0, 1), (0, -1), (1, 1), (1, 0), (-1, 0), (-1, -1)];

fn main() {
    let mut prev: HashSet<(i16, i16)> = include_bytes!("../input.txt")
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
    let mut cur: HashSet<(i16, i16)> = HashSet::new();
    let mut neighbors: HashMap<(i16, i16), usize> = HashMap::new();

    for _ in 0..100 {
        for (x, y) in &prev {
            for (xx, yy) in &NEIGHBORS {
                *neighbors.entry((x + xx, y + yy)).or_default() += 1;
            }
        }

        cur.clear();
        neighbors.drain().for_each(|(coord, count)| {
            let black = prev.contains(&coord);
            if black && count <= 2 || !black && count == 2 {
                cur.insert(coord);
            }
        });
        std::mem::swap(&mut prev, &mut cur);
    }

    println!("{}", prev.len());
}
