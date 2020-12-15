use std::collections::{hash_map::Entry, HashMap};

const COUNT: u32 = 30_000_000;
const BOUNDRY: u32 = COUNT / 10;

pub fn main() {
    let mut input = include_str!("../input.txt")
        .trim_end()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut cur = input.remove(input.len() - 1);
    let mut high: HashMap<u32, u32> = HashMap::with_capacity(1024 * 4);
    let mut low: [u32; BOUNDRY as usize] = [0; BOUNDRY as usize];
    input
        .iter()
        .enumerate()
        .for_each(|(i, &n)| low[n as usize] = i as u32 + 1);

    for i in input.len() as u32..COUNT - 1 {
        if cur < BOUNDRY {
            let lownum = &mut low[cur as usize];
            cur = if *lownum == 0 { 0 } else { i - *lownum + 1 };
            *lownum = i + 1;
        } else {
            match high.entry(cur) {
                Entry::Occupied(mut occup) => cur = i - occup.insert(i),
                Entry::Vacant(vacant) => {
                    vacant.insert(i);
                    cur = 0;
                }
            }
        }
    }

    println!("{}", cur);
}
