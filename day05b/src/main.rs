fn main() {
    let mut ids = include_bytes!("../input.txt")
        .chunks(11)
        .map(|b| {
            b[..10]
                .iter()
                .fold(0, |id, b| (id << 1) | (!b & 4) as usize >> 2)
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();

    println!(
        "{}",
        ids.windows(2).find(|n| n[0] == n[1] - 2).unwrap()[0] + 1,
    );
}
