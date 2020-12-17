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
    let neighbors: Vec<(isize, isize, isize)> = (0..3 * 3 * 3)
        .filter(|&i| i != 3 * 3 * 3 / 2)
        .map(|i| (i % 3 - 1, i / 3 % 3 - 1, i / 9 - 1))
        .collect();

    let mut cur = vec![vec![vec![false; max + 1]; max + 1]; max / 2 + 1];
    let mut prev = cur.clone();
    for x in 0..base_size {
        for y in 0..base_size {
            cur[0][origin - base_size / 2 + x][origin - base_size / 2 + y] = slice[x][y];
        }
    }

    for cycle in 0..CYCLES {
        std::mem::swap(&mut cur, &mut prev);

        let size = base_size + cycle * 2;
        for x in 0..=cycle + 1 {
            for y in 0..=size {
                for z in 0..=size {
                    let (y, z) = (origin - size / 2 + y, origin - size / 2 + z);
                    let o = neighbors
                        .iter()
                        .map(|&r| (
                            (x as isize + r.0).abs() as usize,
                            (y as isize + r.1) as usize,
                            (z as isize + r.2) as usize,
                        ))
                        .filter(|(x, y, z)| prev[*x][*y][*z])
                        .count();
                    cur[x][y][z] = if prev[x][y][z] { o == 2 || o == 3 } else { o == 3 };
                }
            }
        }
    }

    println!(
        "{}",
        2 * cur
            .iter()
            .flat_map(|y| y.iter().flat_map(|z| z.iter().filter(|&c| *c)))
            .count()
            - cur[0].iter().flat_map(|z| z.iter().filter(|&c| *c)).count()
    );
}
