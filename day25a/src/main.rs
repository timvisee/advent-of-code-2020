#![feature(str_split_once)]

#[rustfmt::skip]
fn main() {
    let (d, c) = include_str!("../input.txt").trim().split_once('\n').unwrap();
    let (d, c): (u64, u64) = (d.parse().unwrap(), c.parse().unwrap());

    let sec = std::iter::successors(Some(1), |sec| Some(*sec * 7 % 20201227))
        .position(|sec| sec == d)
        .unwrap();
    println!("{}", (0..sec).fold(1, |i, _| i * c % 20201227));
}
