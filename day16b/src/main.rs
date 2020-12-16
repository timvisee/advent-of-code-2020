use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let re = Regex::new(r#"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$"#).unwrap();
    let input = include_str!("../input.txt");

    let rules: Vec<(usize, usize, usize, usize, &str)> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|rule| {
            let cap = re.captures(rule).unwrap();
            (
                cap[2].parse::<usize>().unwrap(),
                cap[3].parse::<usize>().unwrap(),
                cap[4].parse::<usize>().unwrap(),
                cap[5].parse::<usize>().unwrap(),
                cap.get(1).unwrap().as_str(),
            )
        })
        .collect();

    let my_ticket = tickets("your ticket:", &input).next().unwrap();
    let tickets: Vec<_> = tickets("nearby tickets:", &input)
        .filter(|t| {
            t.iter().all(|&f| {
                rules
                    .iter()
                    .any(|r| (f >= r.0 && f <= r.1) || (f >= r.2 && f <= r.3))
            })
        })
        .collect();

    #[rustfmt::skip]
    let mut rule_fields: HashMap<_, HashSet<_>> = rules
        .iter()
        .map(|f| (
            f.4, (0..tickets[0].len())
                .filter(|i| {
                    tickets
                        .iter()
                        .map(|t| t[*i])
                        .filter(|&v| (v >= f.0 && v <= f.1) || (v >= f.2 && v <= f.3))
                        .count()
                        == tickets.len()
                })
                .collect(),
        ))
        .collect();

    let mut map = vec![""; rules.len()];
    for _ in 0..map.len() {
        let (&f, p) = rule_fields
            .iter()
            .filter(|(_, p)| p.len() == 1)
            .next()
            .unwrap();
        let name = f;
        let pos = p.iter().next().unwrap().clone();
        map[pos] = name;
        rule_fields.values_mut().for_each(|p| {
            p.remove(&pos);
        });
        rule_fields.remove(&name);
    }

    println!(
        "{}",
        my_ticket
            .into_iter()
            .enumerate()
            .map(|(i, v)| (map[i], v))
            .filter(|(f, _)| f.starts_with("dep"))
            .map(|(_, v)| v)
            .product::<usize>()
    );
}

fn tickets(header: &'static str, input: &'static str) -> impl Iterator<Item = Vec<usize>> {
    input
        .lines()
        .skip_while(move |&t| t != header)
        .skip(1)
        .take_while(|t| !t.is_empty())
        .map(|t| t.split(',').map(|f| f.parse().unwrap()).collect())
}
