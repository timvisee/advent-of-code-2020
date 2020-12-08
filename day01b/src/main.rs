pub fn main() {
    let mut items: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    items.sort_unstable();

    for a in &items {
        for b in &items {
            for c in &items {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}
