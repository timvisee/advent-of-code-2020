fn main() {
    let mut ids = std::fs::read("./input.txt")
        .unwrap()
        .chunks(11)
        .map(|b| {
            b[..10].iter().fold(0, |mut id, b| {
                id <<= 1;
                id |= (!b & 0b100) as usize >> 2;
                id
            })
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();

    println!(
        "{}",
        ids.windows(2).find(|n| n[0] == n[1] - 2).unwrap()[0] + 1,
    );
}
