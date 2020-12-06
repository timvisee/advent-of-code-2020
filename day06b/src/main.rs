use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| (
                g.lines().count(),
                g.chars()
                    .filter(|c| c != &'\n')
                    .fold(HashMap::new(), |mut map, c| {
                        *map.entry(c).or_insert(0) += 1;
                        map
                    })
            ))
            .map(|(count, g)| ('a'..='z')
                .filter(|c| g.get(c).map(|c| c == &count).unwrap_or(false))
                .count())
            .sum::<usize>()
    );
}
