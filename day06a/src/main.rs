pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|g| g
                .bytes()
                .filter(|b| b != &b'\n')
                .fold(0u32, |map, b| map | 1 << b - b'a')
                .count_ones())
            .sum::<u32>()
    );
}
