fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| g
                .lines()
                .map(|l| l.bytes().fold([0; 26], |mut map, b| {
                    map[(b - b'a') as usize] += 1;
                    map
                }))
                .fold((0, [0; 26]), |(count, mut map), g| {
                    (0..26).for_each(|i| map[i] += g[i]);
                    (count + 1, map)
                }))
            .map(|(count, g)| g.iter().filter(|v| v == &&count).count())
            .sum::<usize>()
    );
}
