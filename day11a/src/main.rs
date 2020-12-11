const C: isize = 98;
const R: isize = 97;

const NO: u8 = 0;
const EM: u8 = 1;
const OC: u8 = 2;

pub fn main() {
    // Find seat indices
    let seats: Vec<usize> = include_bytes!("../input.txt")
        .into_iter()
        .filter(|b| b != &&b'\n')
        .enumerate()
        .filter(|(_, s)| **s == b'L')
        .map(|(i, _)| i)
        .collect();

    // Create seat map, mark seat positions as empty
    let mut cur = [NO; (R * C) as usize];
    for i in &seats {
        cur[*i] = EM;
    }
    let mut prev = cur;

    // Track seat neighbours indices
    #[rustfmt::skip]
    let seats: Vec<(usize, Vec<usize>)> = seats
        .iter()
        .map(|i| (*i, (0..9)
            .filter(|r| r != &4)
            .map(|r| ((*i as isize % C) + r % 3 - 1, (*i as isize / C) + r / 3 - 1))
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < C && *y < R)
            .map(|(x, y)| (y * C + x) as usize)
            .filter(|i| cur[*i] == EM)
            .collect(),
        ))
        .collect();

    while {
        for (i, visible) in &seats {
            let occup = visible.iter().filter(|i| prev[**i] == OC).count();
            let (cur_seat, prev_seat) = (&mut cur[*i], prev[*i]);

            if prev_seat == EM && occup == 0 {
                *cur_seat = OC;
            } else if prev_seat == OC && occup >= 4 {
                *cur_seat = EM;
            } else {
                *cur_seat = prev_seat;
            }
        }

        std::mem::swap(&mut cur, &mut prev);
        cur != prev
    } {}

    println!("{}", cur.iter().filter(|s| **s == OC).count());
}
