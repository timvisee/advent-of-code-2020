fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let map: Vec<&[u8]> = data.lines().map(|l| l.as_bytes()).collect();

    println!(
        "{}",
        (0..5)
            .map(|i| ((1 + i * 2) % 8, 1 + i / 4))
            .map(|(xx, yy)| (0..)
                .map(|i| ((i + 1) * xx, (i + 1) * yy))
                .take_while(|(_, y)| y < &map.len())
                .filter(|(x, y)| map[*y][*x % map[0].len()] == b'#')
                .count())
            .product::<usize>()
    );
}
