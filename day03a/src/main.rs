fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let map: Vec<&[u8]> = data.lines().map(|l| l.as_bytes()).collect();

    println!(
        "{}",
        (0..map.len() - 1)
            .map(|i| ((i + 1) * 3, i + 1))
            .filter(|(x, y)| map[*y][*x % map[0].len()] == b'#')
            .count()
    );
}
