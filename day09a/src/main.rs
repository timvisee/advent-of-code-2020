pub fn main() {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    println!(
        "{}",
        nums.windows(26)
            .find(|nums| {
                let tot = nums[25];
                for i in 0..24 {
                    for j in (i + 1)..25 {
                        if nums[i] + nums[j] == tot {
                            return false;
                        }
                    }
                }
                true
            })
            .unwrap()[25]
    );
}
