#![feature(str_split_once)]

use std::collections::HashMap;

const SIZE: usize = 10;

#[rustfmt::skip]
pub fn main() {
    let tiles = include_str!("../input.txt")
        .split("\n\n")
        .map(|data| {
            let (id, cells) = data.split_once('\n').unwrap();
            let id = id[5..].trim_end_matches(":").parse().unwrap();
            let cells: Vec<u8> = cells.bytes().filter(|&b| b != b'\n').collect();
            (id, [
                cells[0..SIZE].into_iter()
                    .fold(0u16, |e, &c| e << 1 | (c == b'#') as u16),
                (0..SIZE).map(|i| cells[SIZE - 1 + i * SIZE])
                    .fold(0u16, |e, c| e << 1 | (c == b'#') as u16),
                cells[SIZE * (SIZE - 1)..SIZE * SIZE].into_iter()
                    .fold(0u16, |e, &c| e << 1 | (c == b'#') as u16),
                (0..SIZE).map(|i| cells[i * SIZE])
                    .fold(0u16, |e, c| e << 1 | (c == b'#') as u16),
            ])
        })
        .collect::<Vec<(usize, [u16; 4])>>();

    let edges: HashMap<u16, u8> = tiles
        .iter()
        .flat_map(|t| t.1.iter().map(|&e| [e, e.reverse_bits() >> 6]))
        .fold(HashMap::new(), |mut map, [a, b]| {
            *map.entry(a).or_default() += 1;
            *map.entry(b).or_default() += 1;
            map
        });

    println!(
        "{}",
        tiles
            .iter()
            .filter(|t| t.1.iter().filter(|e| edges[e] > 1).take(3).count() < 3)
            .map(|t| t.0)
            .product::<usize>()
    );
}
