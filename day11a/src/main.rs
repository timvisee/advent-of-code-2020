const COLS: usize = 98;
const ROWS: usize = 97;

pub fn main() {
    let seats: Vec<(usize, usize)> = include_bytes!("../input.txt")
        .into_iter()
        .filter(|b| b != &&b'\n')
        .enumerate()
        .filter(|(_, s)| **s == b'L' && **s != b'\n')
        .map(|(i, _)| (i % COLS + 1, i / COLS + 1))
        .collect();

    let mut cur = [[false; COLS + 2]; ROWS + 2];
    let mut prev = cur;

    while {
        for (x, y) in &seats {
            let occup = (0..9)
                .filter(|i| i != &4)
                .map(|i| (i % 3, i / 3))
                .filter(|(xx, yy)| prev[y + yy - 1][x + xx - 1])
                .count();

            let cur_seat = &mut cur[*y][*x];
            let prev_seat = prev[*y][*x];

            if !prev_seat && occup == 0 {
                *cur_seat = true;
            } else if prev_seat && occup >= 4 {
                *cur_seat = false;
            } else {
                *cur_seat = prev_seat;
            }
        }

        std::mem::swap(&mut cur, &mut prev);

        cur != prev
    } {}

    println!(
        "{}",
        cur.iter()
            .map(|s| s.iter().filter(|s| **s).count())
            .sum::<usize>()
    );
}
