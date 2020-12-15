pub fn main() {
    let mut nums = include_str!("../input.txt")
        .trim_end()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    nums.reserve(2020);

    while nums.len() < 2020 {
        let last = nums.last().unwrap();
        match nums
            .iter()
            .rev()
            .enumerate()
            .skip(1)
            .find(|(_, n)| n == &last)
        {
            Some((i, _)) => nums.push(i),
            None => nums.push(0),
        }
    }

    println!("{}", nums.last().unwrap());
}
