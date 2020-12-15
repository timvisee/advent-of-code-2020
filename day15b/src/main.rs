use std::collections::{hash_map::Entry, HashMap};

const COUNT: usize = 30_000_000;
const LOWCOUNT: usize = COUNT / 10;

pub fn main() {
    let mut input = include_str!("../input.txt")
        .trim_end()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut cur = input.remove(input.len() - 1);
    let mut high: HashMap<usize, usize> = HashMap::with_capacity(1024 * 4);
    let mut low: [usize; LOWCOUNT] = unsafe { std::mem::zeroed() };
    input.iter().enumerate().for_each(|(i, &n)| low[n] = i + 1);

    for i in input.len()..COUNT - 1 {
        if cur < LOWCOUNT {
            let lownum = &mut low[cur];
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
