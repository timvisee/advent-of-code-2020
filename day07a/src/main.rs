use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^([a-z ]+) bags contain (.*)$"#).unwrap();
    static ref RE_CONT: Regex = Regex::new(r#"(\d) ([a-z ]+) bags?"#).unwrap();
}

fn main() {
    let list: Vec<_> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .filter(|rule| !rule.ends_with("no other bags."))
        .map(|rule| parse_bag(rule))
        .collect();

    let (mut targets, mut checked) = (vec!["shiny gold"], vec![]);

    while !targets.is_empty() {
        let target = targets.remove(0);
        let extra = list
            .iter()
            .filter(|(_, cont)| cont.contains_key(target))
            .map(|(color, _)| color.as_str())
            .collect::<Vec<_>>();

        for e in extra {
            if !checked.contains(&e) {
                targets.push(&e);
                checked.push(&e);
            }
        }
    }

    println!("{}", checked.len());
}

fn parse_bag(rule: &str) -> (String, HashMap<String, usize>) {
    let captures = RE_RULE.captures(rule).unwrap();
    (captures[1].into(), parse_contents(captures[2].into()))
}

fn parse_contents(contents: &str) -> HashMap<String, usize> {
    RE_CONT
        .captures_iter(contents)
        .map(|cond| (cond[2].into(), cond[1].parse().unwrap()))
        .collect()
}
