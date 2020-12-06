fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| g
                .lines()
                .map(|l| l.bytes().fold(0u32, |map, b| map | 1 << b - b'a'))
                .fold(std::u32::MAX, |map, g| map & g)
                .count_ones())
            .sum::<u32>()
    );
}
