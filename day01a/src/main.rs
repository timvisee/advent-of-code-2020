use itertools::Itertools;

fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|i| i.parse::<usize>().unwrap())
            .combinations(2)
            .filter(|i| i.iter().sum::<usize>() == 2020)
            .next()
            .map(|i| i.iter().product::<usize>())
            .unwrap()
    );
}
