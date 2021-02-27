const FIND: usize = 22406676;

pub fn main() {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    let (mut tail, mut head, mut tot) = (0, 1, nums[0] + nums[1]);
    while tot != FIND {
        while tot < FIND {
            head += 1;
            tot += nums[head];
        }
        while tot > FIND {
            tot -= nums[tail];
            tail += 1;
        }
    }

    let set = &nums[tail..=head];
    println!("{}", set.iter().min().unwrap() + set.iter().max().unwrap());
}
