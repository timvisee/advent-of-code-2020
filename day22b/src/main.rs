#![feature(str_split_once)]

use std::collections::HashSet;

const ALLOC_SIZE: usize = 3400;

#[rustfmt::skip]
fn main() {
    let (p1, p2) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let p1 = Deck::from(&p1.lines().skip(1).map(|n| n.parse().unwrap()).collect::<Vec<_>>());
    let p2 = Deck::from(&p2.lines().skip(1).map(|n| n.parse().unwrap()).collect::<Vec<_>>());

    println!(
        "{}",
        if play(&p1, &p2) { p1 } else { p2 }
            .as_slice()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * c)
            .sum::<usize>()
    );
}

fn play(p1: &Deck, p2: &Deck) -> bool {
    let mut turns = HashSet::new();
    while turns.insert((p1.as_slice(), p2.as_slice())) {
        if p1.len() == 0 || p2.len() == 0 {
            return p1.len() > 0;
        }

        let (c1, c2) = (p1.pop(), p2.pop());
        if if c1 <= p1.len() && c2 <= p2.len() {
            let sp1 = Deck::from(&p1.as_slice()[..c1]);
            let sp2 = Deck::from(&p2.as_slice()[..c2]);
            sp1.as_slice().iter().max() >= sp2.as_slice().iter().max() || play(&sp1, &sp2)
        } else {
            c1 > c2
        } {
            p1.push(c1);
            p1.push(c2);
        } else {
            p2.push(c2);
            p2.push(c1);
        }
    }

    true
}

struct Deck(usize, usize, [usize; ALLOC_SIZE]);

impl Deck {
    fn from(slice: &[usize]) -> Self {
        let mut cards = [0; ALLOC_SIZE];
        cards[..slice.len()].copy_from_slice(&slice);
        Deck(0, slice.len(), cards)
    }

    fn as_slice(&self) -> &[usize] {
        &self.2[self.0..self.1]
    }

    fn len(&self) -> usize {
        self.1 - self.0
    }

    fn push(&self, card: usize) {
        unsafe {
            (*(&self.2 as *const _ as *mut [_; ALLOC_SIZE]))[self.1] = card;
            *(&self.1 as *const _ as *mut usize) += 1;
        }
    }

    fn pop(&self) -> usize {
        unsafe {
            *(&self.0 as *const _ as *mut usize) += 1;
        }
        self.2[self.0 - 1]
    }
}
