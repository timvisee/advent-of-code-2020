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
        .into_iter()
        .map(|i| (i, (0..9)
            .filter(|r| r != &4)
            .map(|r| (r % 3 - 1, r / 3 - 1))
            .filter_map(|(rx, ry)| (1..)
                .map(|f| ((i as isize % C) + rx * f, (i as isize / C) + ry * f))
                .take_while(|(x, y)| *x >= 0 && *y >= 0 && *x < C && *y < R)
                .map(|(x, y)| (y * C + x) as usize)
                .filter(|i| cur[*i] == EM)
                .next()
            )
            .collect(),
        ))
        .collect();

    while {
        for (i, visible) in &seats {
            let occup = visible.iter().filter(|i| prev[**i] == OC).count();
            let (cur_seat, prev_seat) = (&mut cur[*i], prev[*i]);

            if prev_seat == EM && occup == 0 {
                *cur_seat = OC;
            } else if prev_seat == OC && occup >= 5 {
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
