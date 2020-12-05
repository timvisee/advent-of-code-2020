fn main() {
    println!(
        "{}",
        std::fs::read("./input.txt")
            .unwrap()
            .chunks(11)
            .map(|b| b[..10].iter().fold(0, |mut id, b| {
                id <<= 1;
                id |= (!b & 0b100) as usize >> 2;
                id
            }))
            .max()
            .unwrap(),
    );
}
