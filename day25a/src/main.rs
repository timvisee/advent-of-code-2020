#![feature(str_split_once)]

#[rustfmt::skip]
fn main() {
    let (d, c) = include_str!("../input.txt").trim().split_once('\n').unwrap();
    let (d, c): (u64, u64) = (d.parse().unwrap(), c.parse().unwrap());

    let (mut pk1, mut pk2, mut ek1, mut ek2) = (1, 1, 1, 1);
    loop {
        pk1 = pk1 * 7 % 20201227;
        pk2 = pk2 * 7 % 20201227;
        ek1 = ek1 * d % 20201227;
        ek2 = ek2 * c % 20201227;
        if pk1 == c {
            println!("{}", ek1);
            return;
        } else if pk2 == d {
            println!("{}", ek2);
            return;
        }
    }
}
