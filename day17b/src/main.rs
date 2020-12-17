const CYCLES: usize = 6;

#[rustfmt::skip]
pub fn main() {
    let slice: Vec<Vec<bool>> = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|row| row.into_iter().map(|&b| b == b'#').collect())
        .collect();

    let base_size = slice.len();
    let max = CYCLES * 2 + base_size;
    let origin = max / 2;
    let neighbors: Vec<(isize, isize, isize, isize)> = (0..3 * 3 * 3 * 3)
        .filter(|&i| i != 3 * 3 * 3 * 3 / 2)
        .map(|i| (i % 3 - 1, i / 3 % 3 - 1, i / 9 % 3 - 1, i / 27 - 1))
        .collect();

    let mut cur = vec![vec![vec![vec![false; max + 1]; max + 1]; max / 2 + 1]; max / 2 + 1];
    let mut prev = cur.clone();
    for x in 0..base_size {
        for y in 0..base_size {
            cur[0][0][origin - base_size / 2 + x][origin - base_size / 2 + y] = slice[x][y];
        }
    }

    for cycle in 0..CYCLES {
        std::mem::swap(&mut cur, &mut prev);

        let size = base_size + cycle * 2;
        for x in 0..=cycle + 1 {
            for y in 0..=cycle + 1 {
                for z in 0..=size {
                    for w in 0..size {
                        let (z, w) = (origin - size / 2 + z, origin - size / 2 + w);
                        let o = neighbors
                            .iter()
                            .map(|&r| (
                                (x as isize + r.0).abs() as usize,
                                (y as isize + r.1).abs() as usize,
                                (z as isize + r.2) as usize,
                                (w as isize + r.3) as usize,
                            ))
                            .filter(|(x, y, z, w)| prev[*x][*y][*z][*w])
                            .count();
                        cur[x][y][z][w] = if prev[x][y][z][w] { o == 2 || o == 3 } else { o == 3 };
                    }
                }
            }
        }
    }

    println!(
        "{}",
        4 * (cur.iter()
                .map(|y| {
                    y.iter()
                        .flat_map(|z| z.iter().flat_map(|w| w.iter().filter(|&c| *c)))
                        .count()
                })
                .sum::<usize>()
                - cur
                    .iter()
                    .map(|x| x[0].iter().flat_map(|z| z.iter().filter(|&c| *c)).count())
                    .sum::<usize>())
    );
}
