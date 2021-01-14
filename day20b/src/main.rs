#![feature(str_split_once, array_windows)]

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::mem;

type Pos = (usize, usize);
type Edge = u16;
type Empties = HashSet<Pos>;
type Tiles = Vec<Tile>;
type EdgeCounts = HashMap<Edge, u8>;
type Field = [[Option<Tile>; FIELD_SIZE]; FIELD_SIZE];

const SIZE: usize = 10;
const FIELD_SIZE: usize = 12;
const NEIGHBORS: [Pos; 4] = [(0, usize::MAX), (1, 0), (0, 1), (usize::MAX, 0)];
#[rustfmt::skip]
const MONSTER: [Pos; 15] = [(0, 1), (1, 2), (4, 2), (5, 1), (6, 1), (7, 2), (10, 2), (11, 1), (12, 1), (13, 2), (16, 2), (17, 1), (18, 0), (18, 1), (19, 1)];

pub fn main() {
    let mut tiles = include_str!("../input.txt")
        .split("\n\n")
        .map(|d| Tile::from(d))
        .collect::<Vec<Tile>>();

    // Count edge occurences
    let edges: EdgeCounts = tiles
        .iter()
        .flat_map(|t| t.edges.iter().map(|e| [*e, e.reverse_bits() >> 6]))
        .fold(HashMap::new(), |mut map, [a, b]| {
            *map.entry(a).or_default() += 1;
            *map.entry(b).or_default() += 1;
            map
        });

    // Find corner
    let i = tiles
        .iter()
        .position(|t| t.edges.iter().filter(|e| edges[*e] > 1).take(3).count() < 3)
        .unwrap();
    let mut tile = tiles.remove(i);

    // Find first edge with one occurrence, rotate it to become edge 3 (top left piece)
    let edge_side = tile
        .edges
        .iter()
        .cycle()
        .map(|e| edges[e] < 2)
        .take(5)
        .collect::<Vec<_>>()
        .array_windows()
        .position(|[a, b]| *a && *b)
        .unwrap();
    let corner_rot = 3 - edge_side;
    tile.rotate(corner_rot);
    tile.orient_body(corner_rot);

    assert!(edges[&tile.edges[0]] == 1);
    assert!(edges[&tile.edges[3]] == 1);

    let mut field: Field = Default::default();
    let mut empties = Empties::new();

    place_at(&mut field, tile, (0, 0), &mut empties);

    'test: while !empties.is_empty() {
        // Get random empty tile
        // let empty = *empties.iter().next().unwrap();

        for empty in &empties.clone() {
            if fit_at(&mut field, *empty, &mut tiles, &mut empties, &edges) {
                continue 'test;
            }
        }

        panic!("FAILED");
    }

    // Check
    // TODO: remove
    let product = field[0][0].as_ref().unwrap().id
        * field[FIELD_SIZE - 1][0].as_ref().unwrap().id
        * field[0][FIELD_SIZE - 1].as_ref().unwrap().id
        * field[FIELD_SIZE - 1][FIELD_SIZE - 1].as_ref().unwrap().id;
    assert!(
        product == 20899048083289 || product == 108603771107737,
        "corner checksum failed!"
    );

    let body_size = SIZE - 2;

    let mut supermap: Vec<Vec<u8>> = field
        .iter()
        .flat_map(|tile_row| {
            (0..body_size).into_iter().map(move |y| {
                tile_row
                    .iter()
                    .flat_map(|tile| tile.as_ref().unwrap().body[y].to_vec())
                    .collect()
            })
        })
        .collect();

    assert_eq!(supermap.len(), body_size * FIELD_SIZE);
    assert_eq!(supermap[0].len(), body_size * FIELD_SIZE);

    let mut subtract = 0;
    for orient in 0..8 {
        flip(&mut supermap);
        if orient % 2 == 1 {
            rot(&mut supermap, 1);
        }

        subtract = count_monsters(&mut supermap) * MONSTER.len();

        if subtract > 0 {
            break;
        }
    }
    assert_ne!(subtract, 0);

    // for y in 0..supermap.len() {
    //     // if y % body_size == 0 {
    //     //     println!();
    //     //     // for tile in &field[y / body_size] {
    //     //     //     let id = tile.as_ref().unwrap().id;
    //     //     //     print!("{:<4}:       ", id);
    //     //     // }
    //     //     // println!();
    //     // }

    //     for x in 0..supermap.len() {
    //         print!("{}", supermap[y][x] as char);

    //         // if x % body_size == body_size - 1 {
    //         //     print!("  ");
    //         // }
    //     }

    //     println!();
    // }
    // println!();

    let roughness: usize = supermap
        .iter()
        .map(|r| r.iter().filter(|&b| b == &b'#').count())
        .sum::<usize>()
        - subtract;
    println!("{}", roughness);
}

fn place_at(field: &mut Field, tile: Tile, at: Pos, empties: &mut Empties) {
    field[at.1][at.0] = Some(tile);

    empties.remove(&at);

    empties.extend(
        [(at.0 + 1, at.1), (at.0, at.1 + 1)]
            .iter()
            .filter(|p| p.0 < FIELD_SIZE && p.1 < FIELD_SIZE)
            .filter(|p| field[p.1][p.0].is_none()),
    );
}

fn fit_at(
    field: &mut Field,
    pos: Pos,
    tiles: &mut Tiles,
    empties: &mut Empties,
    edge_counts: &EdgeCounts,
) -> bool {
    // Make sure tile is empty
    assert!(field[pos.1][pos.0].is_none());

    // Other sides that must fit, and their edges to this tile
    let neighbor_pos: Vec<Pos> = NEIGHBORS
        .iter()
        .map(|p| (p.0.wrapping_add(pos.0), p.1.wrapping_add(pos.1)))
        .collect();
    let neighbor_sides: Vec<(usize, Edge)> = neighbor_pos
        .iter()
        .enumerate()
        .filter(|(_, p)| p.0 < FIELD_SIZE && p.1 < FIELD_SIZE)
        .filter_map(|(i, p)| field[p.1][p.0].as_ref().map(|p| (i, p)))
        .map(|(i, t)| (i, t.edges[(i + 2) % 4]))
        .collect();
    let unique_sides = neighbor_pos
        .iter()
        .filter(|p| p.0 >= FIELD_SIZE || p.1 >= FIELD_SIZE)
        .count();

    let (tile_index, tile) = tiles
        .iter()
        .enumerate()
        .filter_map(|(tile_index, tile)| {
            // Return early if unique sides don't match
            if unique_sides != tile.edges.iter().filter(|e| edge_counts[e] == 1).count() {
                return None;
            }

            // Rotate current tile four times until it fits
            for orientation in 0..8 {
                let mut tile: Tile = tile.clone();

                let rot = orientation % 4;
                let flip = orientation >= 4;

                // Prepare orientation
                if flip {
                    tile.flip();
                }
                tile.rotate(rot);

                let fits = neighbor_sides
                    .iter()
                    .all(|(side, edge)| &tile.edges[*side] == edge);
                if !fits {
                    continue;
                }

                tile.orient_body(orientation);

                return Some((tile_index, tile));
            }

            None
        })
        .next()
        .unwrap();

    tiles.remove(tile_index);
    place_at(field, tile, pos, empties);

    true
}

#[derive(Clone)]
struct Tile {
    pub id: usize,

    /// North, east, south, west
    pub edges: [u16; 4],

    /// Individual cells.
    pub body: Vec<Vec<u8>>,
}

impl Tile {
    fn from(data: &str) -> Self {
        let (id, cells) = data.split_once('\n').unwrap();
        let id = id[5..].trim_end_matches(":").parse().unwrap();
        let cells: Vec<u8> = cells.bytes().filter(|&b| b != b'\n').collect();

        // TODO: improve this?
        let body = cells[SIZE..SIZE * (SIZE - 1)]
            .chunks(SIZE)
            .map(|r| r[1..SIZE - 1].to_vec())
            .clone()
            .collect();

        Self {
            id,
            edges: [
                cells[0..SIZE]
                    .into_iter()
                    .fold(0u16, |e, &c| e << 1 | (c == b'#') as u16),
                (0..SIZE)
                    .map(|i| cells[SIZE - 1 + i * SIZE])
                    .fold(0u16, |e, c| e << 1 | (c == b'#') as u16),
                cells[SIZE * (SIZE - 1)..SIZE * SIZE]
                    .into_iter()
                    .fold(0u16, |e, &c| e << 1 | (c == b'#') as u16),
                (0..SIZE)
                    .map(|i| cells[i * SIZE])
                    .fold(0u16, |e, c| e << 1 | (c == b'#') as u16),
            ],
            body,
        }
    }

    /// Flip vertically (x axis).
    fn flip(&mut self) {
        self.edges[1] = self.edges[1].reverse_bits() >> 6;
        self.edges[3] = self.edges[3].reverse_bits() >> 6;
        let (a, b) = self.edges.split_at_mut(2);
        mem::swap(&mut a[0], &mut b[0]);
    }

    /// Rotate once clockwise.
    fn rotate(&mut self, amount: usize) {
        // TODO: remove loop here
        for _ in 0..amount % 4 {
            self.edges = [
                self.edges[3].reverse_bits() >> 6,
                self.edges[0],
                self.edges[1].reverse_bits() >> 6,
                self.edges[2],
            ];
        }
    }

    /// Orientate body once we know proper orientation.
    fn orient_body(&mut self, orientation: usize) {
        if orientation >= 4 {
            flip(&mut self.body);
        }
        rot(&mut self.body, orientation % 4);
    }
}

fn rot<T: Copy>(arr: &mut Vec<Vec<T>>, rot: usize) {
    if rot & 2 > 0 {
        flip(arr);
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

fn flip<T: Copy>(arr: &mut Vec<Vec<T>>) {
    arr.reverse();
}

fn count_monsters(map: &mut Vec<Vec<u8>>) -> usize {
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
                    *a == b'#'
                        && *b == b'#'
                        && *c == b'#'
                        && MONSTER.iter().all(|(a, b)| map[b + y][a + x] == b'#')
                })
                .count()
        })
        .for_each(|n| count.set(count.get() + n));
    count.get()
}
