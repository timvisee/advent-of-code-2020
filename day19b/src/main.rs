#![feature(str_split_once)]

use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../input.txt");

    let mut rules: HashMap<usize, Rule> = input
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            let (n, rule) = l.split_once(": ").unwrap();
            (
                n.parse().unwrap(),
                if rule.starts_with('"') {
                    Rule::Lit(rule.chars().nth(1).unwrap() as u8)
                } else {
                    let parts: Vec<_> = rule.splitn(2, '|').collect();
                    let first = parts[0]
                        .split_terminator(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse().unwrap())
                        .collect();

                    if parts.len() == 1 {
                        Rule::Seq(first)
                    } else {
                        let second = parts[1]
                            .split_terminator(' ')
                            .filter(|n| !n.is_empty())
                            .map(|n| n.parse().unwrap())
                            .collect();
                        Rule::SeqOr(first, second)
                    }
                },
            )
        })
        .collect();

    rules.insert(8, Rule::SeqOr(vec![42], vec![42, 8]));
    rules.insert(11, Rule::SeqOr(vec![42, 31], vec![42, 11, 31]));

    let msgs: Vec<&str> = input.lines().skip_while(|l| !l.trim().is_empty()).collect();

    println!(
        "{}",
        msgs.into_iter()
            .filter(|m| matches(m.as_bytes(), &rules, 0)
                .map(|n| n == m.len())
                .unwrap_or(false))
            .count(),
    );
}

fn matches(msg: &[u8], rules: &HashMap<usize, Rule>, rule: usize) -> Option<usize> {
    if msg.is_empty() {
        return None;
    }

    match &rules[&rule] {
        Rule::Lit(c) => {
            if &msg[0] == c {
                Some(1)
            } else {
                None
            }
        }
        Rule::Seq(seq) => {
            let mut consumed = 0;
            for rule in seq {
                match matches(&msg[consumed..], rules, *rule) {
                    Some(n) => consumed += n,
                    None => return None,
                }
            }
            Some(consumed)
        }
        Rule::SeqOr(first, second) => {
            let mut consumed = 0;
            for rule in first {
                match matches(&msg[consumed..], rules, *rule) {
                    Some(n) => consumed += n,
                    None => {
                        consumed = 0;
                        break;
                    }
                }
            }

            if consumed > 0 {
                return Some(consumed);
            }

            for rule in second {
                match matches(&msg[consumed..], rules, *rule) {
                    Some(n) => consumed += n,
                    None => return None,
                }
            }
            Some(consumed)
        }
    }
}

#[derive(Debug)]
enum Rule {
    Lit(u8),
    Seq(Vec<usize>),
    SeqOr(Vec<usize>, Vec<usize>),
}
