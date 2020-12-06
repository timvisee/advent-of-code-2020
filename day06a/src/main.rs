use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| g.chars().filter(|c| c != &'\n').collect::<HashSet<_>>())
            .map(|g| ('a'..='z').filter(|c| g.contains(c)).count())
            .sum::<usize>()
    );
}
