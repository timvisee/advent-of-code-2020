use regex::Regex;
use std::collections::HashMap;

#[rustfmt::skip]
pub fn main() {
    let re = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();
    let mut mem = HashMap::new();
    let mut float_addrs = vec![];
    let mut whitelist = 0;

    for line in include_str!("../input.txt").lines() {
        if line.starts_with("me") {
            let cap = re.captures(&line).unwrap();
            let addr: usize = cap[1].parse().unwrap();
            let val = cap[2].parse().unwrap();
            for float_addr in &float_addrs {
                mem.insert(addr & whitelist | float_addr, val);
            }
        } else {
            let mut float_base = 0;
            let mut float_bits = vec![];
            float_addrs.clear();
            whitelist = 0;

            line.split(" = ")
                .nth(1)
                .unwrap()
                .bytes()
                .rev()
                .enumerate()
                .for_each(|(i, b)| match b {
                    b'0' => whitelist |= 1 << i,
                    b'1' => float_base |= 1 << i,
                    b'X' => float_bits.push(i),
                    _ => unreachable!(),
                });

            float_addrs = (0..2usize.pow(float_bits.len() as u32))
                .map(|template| float_bits
                    .iter()
                    .enumerate()
                    .fold(float_base, |addr, (i, fb)| addr | (template & 1 << i) << fb - i)
                )
                .collect();
        }
    }

    println!("{}", mem.values().sum::<usize>());
}
