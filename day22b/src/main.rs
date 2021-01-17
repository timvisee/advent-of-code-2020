#![feature(str_split_once)]

use std::collections::{HashSet, VecDeque};

fn main() {
    let (p1, p2) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut p1: VecDeque<usize> = p1.lines().skip(1).map(|n| n.parse().unwrap()).collect();
    let mut p2: VecDeque<usize> = p2.lines().skip(1).map(|n| n.parse().unwrap()).collect();

    let score: usize = if play(&mut p1, &mut p2) {
        p1.iter().rev().enumerate().map(|(i, c)| (i + 1) * c).sum()
    } else {
        p2.iter().rev().enumerate().map(|(i, c)| (i + 1) * c).sum()
    };

    println!("{}", score);
}

fn play(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> bool {
    let mut turns = HashSet::new();
    while turns.insert((p1.clone(), p2.clone())) {
        if p1.len() == 0 || p2.len() == 0 {
            return !p1.is_empty();
        }

        let (c1, c2) = match p1.pop_front().zip(p2.pop_front()) {
            Some(c) => c,
            None => panic!(),
        };

        if c1 <= p1.len() && c2 <= p2.len() {
            if play(
                &mut p1.make_contiguous()[..c1].to_owned().into(),
                &mut p2.make_contiguous()[..c2].to_owned().into(),
            ) {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }

            continue;
        }

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    true
}
