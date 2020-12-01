fn main() {
    let items: Vec<usize> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    for a in &items {
        for b in &items {
            if a + b == 2020 {
                println!("{}", a * b);
                return;
            }
        }
    }
}
