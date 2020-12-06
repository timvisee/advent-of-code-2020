fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| g
                .bytes()
                .filter(|b| b != &b'\n')
                .fold(0, |map, b| map | 1 << b - b'a'))
            .map(u32::count_ones)
            .sum::<u32>()
    );
}
