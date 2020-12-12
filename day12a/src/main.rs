#[rustfmt::skip]
pub fn main() {
    let (x, y, _) = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .fold((0, 0, 0b01000100u8), |(x, y, d), i| {
            let val: isize = atoi::atoi(&i[1..]).unwrap();
            match i[0] {
                b'N' => (x, y - val, d),
                b'S' => (x, y + val, d),
                b'E' => (x + val, y, d),
                b'W' => (x - val, y, d),
                b'L' => (x, y, d.rotate_left(val as u32 / 90)),
                b'R' => (x, y, d.rotate_right(val as u32 / 90)),
                b'F' => if d & 0b0100 != 0 {
                        (x + val, y, d)
                    } else if d & 0b0001 != 0 {
                        (x - val, y, d)
                    } else if d & 0b0010 != 0 {
                        (x, y + val, d)
                    } else if d & 0b1000 != 0 {
                        (x, y - val, d)
                    } else {
                        (x, y, d)
                    }
                _ => unreachable!(),
            }
        });
    println!("{}", x.abs() + y.abs());
}
