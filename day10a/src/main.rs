pub fn main() {
    let mut nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();
    nums.push(0);
    nums.sort_unstable();

    let (one, three) = nums
        .windows(2)
        .fold((0, 1), |(one, three), val| match val[1] - val[0] {
            1 => (one + 1, three),
            3 => (one, three + 1),
            _ => unreachable!(),
        });

    println!("{}", one * three);
}
