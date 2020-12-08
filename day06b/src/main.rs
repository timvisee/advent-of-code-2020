pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|g| g
                .lines()
                .map(|l| l.bytes().fold(0, |map, b| map | 1 << b - b'a'))
                .fold(std::u32::MAX, |map, g| map & g)
                .count_ones())
            .sum::<u32>()
    );
}
