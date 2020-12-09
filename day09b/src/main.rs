const FIND: usize = 22406676;

pub fn main() {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    let max_index = nums.iter().position(|n| n == &FIND).unwrap();

    println!(
        "{}",
        nums[..max_index]
            .windows(26)
            .filter_map(|nums| {
                for size in 2..=26 {
                    let set = &nums[0..size];
                    let sum: usize = set.iter().sum();

                    if sum < FIND {
                        continue;
                    } else if sum > FIND {
                        return None;
                    }

                    return Some(set.iter().min().unwrap() + set.iter().max().unwrap());
                }

                None
            })
            .next()
            .unwrap()
    );
}
