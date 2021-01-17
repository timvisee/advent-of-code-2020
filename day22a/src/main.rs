#![feature(str_split_once)]

use std::collections::VecDeque;

fn main() {
    let (p1, p2) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut p1: VecDeque<usize> = p1.lines().skip(1).map(|n| n.parse().unwrap()).collect();
    let mut p2: VecDeque<usize> = p2.lines().skip(1).map(|n| n.parse().unwrap()).collect();

    while !p2.is_empty() && !p2.is_empty() {
        let (c1, c2) = p1.pop_front().zip(p2.pop_front()).unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    println!(
        "{}",
        p1.iter()
            .chain(p2.iter())
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * c)
            .sum::<usize>()
    );
}
