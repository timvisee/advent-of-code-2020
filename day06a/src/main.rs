fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| g
                .bytes()
                .filter(|b| b != &b'\n')
                .fold([false; 26], |mut map, b| {
                    map[(b - b'a') as usize] = true;
                    map
                }))
            .map(|g| g.iter().filter(|v| **v).count())
            .sum::<usize>()
    );
}
