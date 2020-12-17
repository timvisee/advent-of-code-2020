const CYCLES: usize = 6;

pub fn main() {
    let data = include_bytes!("../input.txt");

    let slice: Vec<Vec<bool>> = data
        .split(|&b| b == b'\n')
        .map(|row| row.into_iter().map(|&b| b == b'#').collect())
        .collect();

    let base_size = slice.len();
    let max_size = CYCLES * 2 + base_size + 3 + 1;
    let origin = max_size / 2;

    let neighbours_rel: Vec<(isize, isize, isize, isize)> = (0..3 * 3 * 3 * 3)
        .filter(|&i| i != 40)
        .map(|i| {
            (
                (i % 3) - 1,
                ((i / 3) % 3) - 1,
                ((i / 9) % 3) - 1,
                i / 27 - 1,
            )
        })
        .collect();

    let mut cur = vec![vec![vec![vec![false; max_size]; max_size]; max_size]; max_size];
    let mut prev = cur.clone();
    for x in 0..base_size {
        for y in 0..base_size {
            cur[origin - base_size / 2 + x][origin - base_size / 2 + y][origin][origin] =
                slice[y][x];
        }
    }

    for cycle in 0..CYCLES {
        std::mem::swap(&mut cur, &mut prev);

        let size = base_size + cycle * 2;

        for x in 0..size + 2 {
            for y in 0..size + 2 {
                for z in 0..size + 2 {
                    for w in 0..size + 2 {
                        let (x, y, z, w) = (
                            origin - size / 2 + x,
                            origin - size / 2 + y,
                            origin - size / 2 + z,
                            origin - size / 2 + w,
                        );

                        let occup = neighbours_rel
                            .iter()
                            .map(|&r| {
                                (
                                    (x as isize + r.0) as usize,
                                    (y as isize + r.1) as usize,
                                    (z as isize + r.2) as usize,
                                    (w as isize + r.3) as usize,
                                )
                            })
                            .filter(|(x, y, z, w)| prev[*x][*y][*z][*w])
                            .count();

                        cur[x][y][z][w] = if prev[x][y][z][w] {
                            occup == 2 || occup == 3
                        } else {
                            occup == 3
                        };
                    }
                }
            }
        }
    }

    println!(
        "{}",
        cur.iter()
            .flat_map(|y| y
                .iter()
                .flat_map(|z| z.iter().flat_map(|w| w.iter().filter(|&c| *c))))
            .count()
    );
}
