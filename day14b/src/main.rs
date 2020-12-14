use regex::Regex;
use std::collections::HashMap;

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

            let bit_len = float_bits.len();
            float_addrs = (0..2usize.pow(bit_len as u32))
                .map(|combi| {
                    (0..bit_len)
                        .filter(|bi| combi & 1 << bi != 0)
                        .fold(float_base, |addr, bi| addr | 1 << float_bits[bi])
                })
                .collect();
        }
    }

    println!("{}", mem.values().sum::<usize>());
}
