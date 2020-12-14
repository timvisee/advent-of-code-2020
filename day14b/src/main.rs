use itertools::Itertools;
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
            let mut addr = captures[1].parse().unwrap();
            let val = captures[2].parse().unwrap();

            let mut floating_bits = vec![];
            mask.chars().enumerate().for_each(|(i, c)| {
                let i = 35 - i;
                match c {
                    '0' => {}
                    '1' => addr |= 1 << i,
                    'X' => floating_bits.push(i),
                    _ => unreachable!(),
                }
            });

            // TODO: do not sort
            floating_bits.sort_unstable();

            // Set all floating bits in base to zero
            for fb in &floating_bits {
                addr &= !(1 << fb);
            }

            let len = floating_bits.len();
            for bits in 0..2usize.pow(len as u32) {
                let mut addr = addr;

                for b in 0..36 {
                    // Set each bit accordingly based on base b iterator mask
                    if (bits & (1 << b)) > 0 {
                        addr |= 1 << floating_bits[b];
                    }
                }

                *mem.entry(addr).or_default() = val;
            }
        }
    }

    println!("{}", mem.values().sum::<usize>());
}
