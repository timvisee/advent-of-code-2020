pub fn main() {
    let mut nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();
    nums.sort_unstable();

    println!(
        "{}",
        2 * nums
            .windows(2)
            .collect::<Vec<_>>()
            .split(|n| n[1] - n[0] == 3)
            .map(|n| match n.len() {
                4 => 7,
                3 => 4,
                2 => 2,
                _ => 1,
            })
            .product::<usize>()
    );
}
