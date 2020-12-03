fn main() {
    let map: Vec<String> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.into())
        .collect();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let width = map[0].len();

    println!(
        "{}",
        slopes
            .iter()
            .map(|(xx, yy)| (0..map.len() - 1)
                .map(|i| ((i + 1) * xx, (i + 1) * yy))
                .filter(|(_, y)| y < &map.len())
                .filter(|(x, y)| map[*y].chars().nth(*x % width).unwrap() == '#')
                .count())
            .product::<usize>()
    );
}
