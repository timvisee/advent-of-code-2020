#[rustfmt::skip]
pub fn main() {
    let (x, y, _) = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .fold((0, 0, 0b01000100u8), |(x, y, d), i| {
            let v: isize = atoi::atoi(&i[1..]).unwrap();
            match i[0] {
                b'N' => (x, y - v, d),
                b'S' => (x, y + v, d),
                b'E' => (x + v, y, d),
                b'W' => (x - v, y, d),
                b'L' => (x, y, d.rotate_left(v as u32 / 90)),
                b'R' => (x, y, d.rotate_right(v as u32 / 90)),
                b'F' => (
                    x + (((d & 4) >> 2) as isize - (d & 1) as isize) * v,
                    y + (((d & 2) >> 1) as isize - ((d & 8) >> 3) as isize) * v,
                    d,
                ),
                _ => unreachable!(),
            }
        });
    println!("{}", x.abs() + y.abs());
}
