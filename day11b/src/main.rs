const COLS: usize = 98;
const ROWS: usize = 97;

const NO: u8 = 0;
const EM: u8 = 1;
const OC: u8 = 2;
type SEAT = u8;

pub fn main() {
    let mut seats: Vec<(usize, usize)> = include_bytes!("../input.txt")
        .into_iter()
        .filter(|b| b != &&b'\n')
        .enumerate()
        .filter(|(_, s)| **s == b'L' && **s != b'\n')
        .map(|(i, _)| (i % COLS + 1, i / COLS + 1))
        .collect();
    seats.sort_unstable();

    let mut cur = [[NO; COLS + 2]; ROWS + 2];

    for (x, y) in &seats {
        cur[*y][*x] = EM;
    }

    let mut prev = cur;

    loop {
        for (x, y) in &seats {
            let occup = around((*x, *y), &prev);

            let cur_seat = &mut cur[*y][*x];
            let prev_seat = prev[*y][*x];

            if prev_seat == EM && occup == 0 {
                *cur_seat = OC;
            } else if prev_seat == OC && occup >= 5 {
                *cur_seat = EM;
            } else {
                *cur_seat = prev_seat;
            }
        }

        std::mem::swap(&mut cur, &mut prev);

        if cur == prev {
            break;
        }
    }

    println!(
        "{}",
        cur.iter()
            .map(|s| s.iter().filter(|s| **s == OC).count())
            .sum::<usize>()
    );
}

#[inline(always)]
fn around((x, y): (usize, usize), map: &[[SEAT; COLS + 2]]) -> usize {
    let mut count = 0;

    for ry in -1isize..=1 {
        for rx in -1isize..=1 {
            if rx == 0 && ry == 0 {
                continue;
            }

            for i in 1.. {
                let (xx, yy) = (x as isize + rx * i, y as isize + ry * i);

                if xx < 0 || yy < 0 {
                    break;
                }

                match map.get(yy as usize) {
                    Some(row) => match row.get(xx as usize) {
                        Some(&OC) => {
                            count += 1;
                            break;
                        }
                        Some(&EM) | None => break,
                        _ => {}
                    },
                    None => break,
                }
            }
        }
    }

    count
}
