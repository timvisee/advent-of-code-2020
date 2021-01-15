#![feature(array_windows, custom_inner_attributes, drain_filter, iterator_fold_self, str_split_once)]
#![rustfmt::skip]

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::mem;

const SIZE: usize = 10;
const FIELD_SIZE: usize = 12;
const NEIGHBORS: [Pos; 4] = [(0, usize::MAX), (1, 0), (0, 1), (usize::MAX, 0)];
const MONSTER: [Pos; 15] = [(0, 1), (1, 2), (4, 2), (5, 1), (6, 1), (7, 2), (10, 2), (11, 1), (12, 1), (13, 2), (16, 2), (17, 1), (18, 0), (18, 1), (19, 1)];

type Pos = (usize, usize);
type Field = [[Option<Tile>; FIELD_SIZE]; FIELD_SIZE];

pub fn main() {
    let mut tiles = include_str!("../input.txt")
        .split("\n\n")
        .map(|d| Tile::from(d))
        .collect::<Vec<Tile>>();

    // Count edge occurences
    // TODO: do not include reverse here, check reverse on lookup?
    let edges: HashMap<u16, Vec<u16>> = tiles
        .iter()
        .fold(HashMap::new(), |mut map, tile| {
            for edge in &tile.edges {
                (*map.entry(*edge).or_default()).push(tile.id);
                (*map.entry(edge.reverse_bits() >> 6).or_default()).push(tile.id);
            }
            map
        });

    // Find a corner, rotate unique edgest to top left
    let i = tiles
        .iter()
        .position(|t| t.edges.iter().filter(|e| edges[*e].len() > 1).take(3).count() < 3)
        .unwrap();
    let mut tile = tiles.remove(i);
    let side = tile
        .edges
        .iter()
        .cycle()
        .map(|e| edges[e].len() < 2)
        .take(5)
        .collect::<Vec<_>>()
        .array_windows()
        .position(|[a, b]| *a && *b)
        .unwrap();
    tile.rot(3 - side);
    tile.orient(3 - side);

    // Create field, place corner, then place the rest
    let mut field: Field = Default::default();
    let mut empties = HashSet::new();
    place(&mut field, tile, (0, 0), &mut empties);
    while !empties.is_empty() {
        fit(&mut field, *empties.iter().next().unwrap(), &mut tiles, &mut empties, &edges);
    }

    // Build waters from tiles
    let mut waters: Vec<Vec<bool>> = field
        .iter()
        .flat_map(|tile_row| {
            (0..SIZE - 2).into_iter().map(move |y| {
                tile_row.iter().flat_map(|t| t.as_ref().unwrap().body[y].to_vec()).collect()
            })
        })
        .collect();

    // Count waves, subtract monster bits
    println!(
        "{}",
        waters
            .iter()
            .map(|r| r.iter().filter(|&&b| b).count())
            .sum::<usize>()
            - (0..8)
                .map(|orient| {
                    waters.reverse();
                    if orient % 2 == 1 {
                        rot(&mut waters, 1);
                    }
                    monsters(&mut waters)
                })
                .filter(|&n| n > 0)
                .next()
                .unwrap()
                * MONSTER.len()
    );
}

#[derive(Clone)]
struct Tile {
    pub id: u16,
    pub edges: [u16; 4],
    pub body: Vec<Vec<bool>>,
}

impl Tile {
    fn from(data: &str) -> Self {
        let (id, cells) = data.split_once('\n').unwrap();
        let id = id[5..].trim_end_matches(":").parse().unwrap();
        let cells: Vec<_> = cells.bytes().filter(|&b| b != b'\n').map(|b| b == b'#').collect();
        Self {
            id,
            edges: [
                cells[0..SIZE].into_iter().fold(0u16, |e, &c| e << 1 | c as u16),
                (0..SIZE).map(|i| cells[SIZE - 1 + i * SIZE]).fold(0u16, |e, c| e << 1 | c as u16),
                cells[SIZE * (SIZE - 1)..SIZE * SIZE].into_iter().fold(0u16, |e, &c| e << 1 | c as u16),
                (0..SIZE).map(|i| cells[i * SIZE]).fold(0u16, |e, c| e << 1 | c as u16),
            ],
            body: cells[SIZE..SIZE * (SIZE - 1)]
                .chunks(SIZE)
                .map(|r| r[1..SIZE - 1].to_vec())
                .collect(),
        }
    }

    fn flip(&mut self) {
        self.edges[1] = self.edges[1].reverse_bits() >> 6;
        self.edges[3] = self.edges[3].reverse_bits() >> 6;
        let (a, b) = self.edges.split_at_mut(2);
        mem::swap(&mut a[0], &mut b[0]);
    }

    fn rot(&mut self, rot: usize) {
        if rot == 1 || rot == 2 {
            self.edges[1] = self.edges[1].reverse_bits() >> 6;
            self.edges[3] = self.edges[3].reverse_bits() >> 6;
        }
        if rot == 2 || rot == 3 {
            self.edges[0] = self.edges[0].reverse_bits() >> 6;
            self.edges[2] = self.edges[2].reverse_bits() >> 6;
        }
        self.edges.rotate_right(rot);
    }

    fn orient(&mut self, orient: usize) {
        if orient >= 4 {
            self.body.reverse();
        }
        rot(&mut self.body, orient % 4);
    }
}

fn place(field: &mut Field, tile: Tile, at: Pos, empties: &mut HashSet<Pos>) {
    field[at.1][at.0] = Some(tile);
    empties.remove(&at);
    empties.extend(
        [(at.0 + 1, at.1), (at.0, at.1 + 1)]
            .iter()
            .filter(|p| p.0 < FIELD_SIZE && p.1 < FIELD_SIZE && field[p.1][p.0].is_none())
    );
}

fn fit(
    field: &mut Field,
    pos: Pos,
    tiles: &mut Vec<Tile>,
    empties: &mut HashSet<Pos>,
    edges: &HashMap<u16, Vec<u16>>,
) {
    let match_edges: Vec<_> = NEIGHBORS
        .iter()
        .map(|p| (p.0.wrapping_add(pos.0), p.1.wrapping_add(pos.1)))
        .enumerate()
        .filter(|(_, p)| p.0 < FIELD_SIZE && p.1 < FIELD_SIZE)
        .filter_map(|(i, p)| field[p.1][p.0].as_ref().map(|p| (i, p)))
        .map(|(i, t)| (i, t.edges[(i + 2) % 4]))
        .collect();

    let (side, edge) = match_edges[0];
    let neighbor_id = field[pos.1 + NEIGHBORS[side].1][pos.0 + NEIGHBORS[side].0].as_ref().unwrap().id;
    let tile_id = *edges[&edge].iter().filter(|&id| id != &neighbor_id).next().unwrap();

    let (i, tile) = tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| tile.id == tile_id)
        .filter_map(|(i, tile)| {
            (0..8)
                .filter_map(|orient| {
                    let mut tile: Tile = tile.clone();
                    if orient >= 4 {
                        tile.flip();
                    }
                    tile.rot(orient % 4);

                    if match_edges.iter().all(|(s, e)| &tile.edges[*s] == e) {
                        tile.orient(orient);
                        Some((i, tile))
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap();

    tiles.remove(i);
    place(field, tile, pos, empties);
}

fn rot<T: Copy>(arr: &mut Vec<Vec<T>>, rot: usize) {
    if rot & 2 > 0 {
        arr.reverse();
        arr.iter_mut().for_each(|r| r.reverse());
    }
    if rot & 1 > 0 {
        let (template, len) = (arr.clone(), arr[0].len());
        for x in 0..len {
            for y in 0..len {
                arr[y][x] = template[len - x - 1][y];
            }
        }
    }
}

fn monsters(map: &mut Vec<Vec<bool>>) -> usize {
    let count = Cell::new(0);
    map.iter()
        .skip(1)
        .enumerate()
        .take(map.len() - 2)
        .take_while(|(i, _)| *i < 3 || count.get() > 0)
        .map(|(y, row)| {
            row.array_windows()
                .skip(17)
                .enumerate()
                .filter(|(x, [a, b, c])| {
                    *a && *b && *c && MONSTER.iter().all(|(a, b)| map[b + y][a + x])
                })
                .count()
        })
        .for_each(|n| count.set(count.get() + n));
    count.get()
}
