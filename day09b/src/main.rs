const FIND: usize = 22406676;

pub fn main() {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    println!(
        "{}",
        nums.windows(20)
            .filter_map(|set| {
                for size in 3..=20 {
                    let set = &set[0..size];
                    let sum: usize = set.iter().sum();

                    if sum < FIND {
                        continue;
                    } else if sum > FIND {
                        return None;
                    }

                    let (min, max) = set
                        .iter()
                        .fold((FIND, 0), |(min, max), v| (min.min(*v), max.max(*v)));
                    return Some(min + max);
                }
                None
            })
            .next()
            .unwrap()
    );
}
