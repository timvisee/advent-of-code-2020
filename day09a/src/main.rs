pub fn main() {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    println!(
        "{}",
        nums.windows(26)
            .find(|set| {
                for i in 0..24 {
                    for j in (i + 1)..25 {
                        if set[i] + set[j] == set[25] {
                            return false;
                        }
                    }
                }
                true
            })
            .unwrap()[25]
    );
}
