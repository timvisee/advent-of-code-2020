const FIND: usize = 22406676;

pub fn main() {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    let (mut start, mut end, mut tot) = (0, 1, nums[0] + nums[1]);

    while tot != FIND {
        while tot < FIND {
            end += 1;
            tot += nums[end];
        }
        while tot > FIND {
            tot -= nums[start];
            start += 1;
        }
    }

    let set = &nums[start..=end];
    println!("{}", set.iter().min().unwrap() + set.iter().max().unwrap());
}
