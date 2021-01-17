#![feature(str_split_once)]

use std::collections::{HashSet, VecDeque};

fn main() {
    let (p1, p2) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut p1 = p1.lines().skip(1).map(|n| n.parse().unwrap()).collect();
    let mut p2 = p2.lines().skip(1).map(|n| n.parse().unwrap()).collect();

    println!(
        "{}",
        if play(&mut p1, &mut p2) { p1 } else { p2 }
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * c)
            .sum::<usize>()
    );
}

fn play(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> bool {
    let mut turns = HashSet::new();
    while turns.insert((p1.clone(), p2.clone())) {
        if p1.len() == 0 || p2.len() == 0 {
            return !p1.is_empty();
        }

        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        if if c1 <= p1.len() && c2 <= p2.len() {
            let mut sp1: VecDeque<_> = p1.make_contiguous()[..c1].to_owned().into();
            let mut sp2: VecDeque<_> = p2.make_contiguous()[..c2].to_owned().into();
            sp1.iter().max() >= sp2.iter().max() || play(&mut sp1, &mut sp2)
        } else {
            c1 > c2
        } {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    true
}
