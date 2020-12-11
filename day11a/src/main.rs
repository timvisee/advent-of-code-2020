const COL: isize = 98;
const ROW: isize = 97;

const NONE: u8 = 0;
const EMPTY: u8 = 1;
const FULL: u8 = 2;

pub fn main() {
    // Find seat indices
    let seats: Vec<usize> = include_bytes!("../input.txt")
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s == &&b'L')
        .map(|(i, _)| i - (i / (COL as usize + 1)))
        .collect();

    // Create seat map, mark seat positions as empty
    let mut cur = [NONE; (ROW * COL) as usize];
    for i in &seats {
        cur[*i] = EMPTY;
    }
    let mut prev = cur;

    // Track seat neighbours indices
    #[rustfmt::skip]
    let seats: Vec<(usize, Vec<usize>)> = seats
        .iter()
        .map(|i| (*i, (0..9)
            .filter(|r| r != &4)
            .map(|r| ((*i as isize % COL) + r % 3 - 1, (*i as isize / COL) + r / 3 - 1))
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < COL && *y < ROW)
            .map(|(x, y)| (y * COL + x) as usize)
            .filter(|i| cur[*i] == EMPTY)
            .collect(),
        ))
        .collect();

    while {
        for (i, visible) in &seats {
            let occup = visible.iter().filter(|i| prev[**i] == FULL).count();
            let (cur_seat, prev_seat) = (&mut cur[*i], prev[*i]);

            if prev_seat == EMPTY && occup == 0 {
                *cur_seat = FULL;
            } else if prev_seat == FULL && occup >= 4 {
                *cur_seat = EMPTY;
            } else {
                *cur_seat = prev_seat;
            }
        }

        std::mem::swap(&mut cur, &mut prev);
        cur != prev
    } {}

    println!("{}", cur.iter().filter(|s| s == &&FULL).count());
}
