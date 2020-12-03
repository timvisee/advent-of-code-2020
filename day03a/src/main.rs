fn main() {
    let map: Vec<String> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.into())
        .collect();

    let width = map[0].len();

    println!(
        "{}",
        (0..map.len() - 1)
            .map(|i| ((i + 1) * 3, i + 1))
            .filter(|(x, y)| map[*y].chars().nth(*x % width).unwrap() == '#')
            .count()
    );
}
