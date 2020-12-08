pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .chunks(11)
            .map(|b| b[..10]
                .iter()
                .fold(0, |id, b| (id << 1) | (!b & 4) as usize >> 2))
            .max()
            .unwrap(),
    );
}
