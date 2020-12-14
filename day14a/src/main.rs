use regex::Regex;
use std::collections::HashMap;

pub fn main() {
    let re = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut and_or = (0, 0);

    for line in include_str!("../input.txt").lines() {
        if line.starts_with("ma") {
            and_or = line
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .bytes()
                .rev()
                .enumerate()
                .fold((usize::MAX, 0), |(and, or), (i, b)| match b {
                    b'0' => (and & !(1 << i), or),
                    b'1' => (and, or | 1 << i),
                    _ => (and, or),
                });
        } else {
            let captures = re.captures(&line).unwrap();
            mem.insert(
                captures[1].parse().unwrap(),
                captures[2].parse::<usize>().unwrap() & and_or.0 | and_or.1,
            );
        }
    }

    println!("{}", mem.values().sum::<usize>());
}
