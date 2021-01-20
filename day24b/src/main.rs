#![feature(array_windows, split_inclusive)]

use std::collections::HashMap;

const NEIGHBORS: [(i32, i32); 6] = [(0, 1), (0, -1), (1, 1), (1, 0), (-1, 0), (-1, -1)];

fn main() {
    let map = include_bytes!("../input.txt")
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
        .fold(HashMap::<_, bool>::new(), |mut map, coord| {
            map.entry(coord).and_modify(|b| *b = !*b).or_insert(true);
            map
        });

    println!(
        "{}",
        (0..100)
            .fold(map, |map, _| cycle(&map))
            .values()
            .filter(|&flip| *flip)
            .count()
    );
}

#[inline(always)]
#[rustfmt::skip]
fn cycle(old: &HashMap<(i32, i32), bool>) -> HashMap<(i32, i32), bool> {
    let mut new = HashMap::new();
    for ((x, y), _) in old.into_iter().filter(|t| *t.1) {
        // If 1 or 2 black neighbors, stay black
        if NEIGHBORS
            .iter()
            .map(|(xx, yy)| (x + xx, y + yy))
            .filter(|coord| *old.get(coord).unwrap_or(&false))
            .take(3)
            .count() - 1 & !1 == 0 {
            new.insert((*x, *y), true);
        }

        // For each white neighbor, if 2 black neighbors, turn black
        NEIGHBORS
            .iter()
            .map(|(xx, yy)| (x + xx, y + yy))
            .filter(|(x, y)| {
                !old.get(&(*x, *y)).unwrap_or(&false)
                    && NEIGHBORS
                        .iter()
                        .map(|(xx, yy)| (x + xx, y + yy))
                        .filter(|coord| *old.get(coord).unwrap_or(&false))
                        .take(3)
                        .count()
                        == 2
            })
            .for_each(|coord| {
                new.insert(coord, true);
            });
    }
    new
}
