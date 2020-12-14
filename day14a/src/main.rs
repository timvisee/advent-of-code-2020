use regex::Regex;
use std::collections::HashMap;

pub fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();

    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask = "";

    let re = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();

    for line in lines {
        if line.starts_with("ma") {
            mask = line.split(" = ").skip(1).next().unwrap();
        } else {
            let captures = re.captures(&line).unwrap();
            let i = captures[1].parse().unwrap();
            let mut val = captures[2].parse().unwrap();

            mask.chars()
                .enumerate()
                .filter(|(_, c)| c != &'X')
                .for_each(|(i, c)| {
                    let i = 35 - i;
                    match c {
                        '0' => val &= !(1 << i),
                        '1' => val |= 1 << i,
                        _ => unreachable!(),
                    }
                });

            *mem.entry(i).or_default() = val;
        }
    }

    println!("{}", mem.values().sum::<usize>());
}
