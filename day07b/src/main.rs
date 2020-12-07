use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^([a-z ]+) bags contain (.*)$"#).unwrap();
    static ref RE_CONT: Regex = Regex::new(r#"(\d) ([a-z ]+) bags?"#).unwrap();
}

fn main() {
    let list: HashMap<_, _> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .filter(|rule| !rule.ends_with("no other bags."))
        .map(|rule| parse_bag(rule))
        .collect();

    println!("{}", bags("shiny gold", &list));
}

fn bags(color: &str, list: &HashMap<String, HashMap<String, usize>>) -> usize {
    if let Some(contents) = list.get(color) {
        contents
            .iter()
            .map(|(color, count)| (bags(color, &list) + 1) * count)
            .sum()
    } else {
        0
    }
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
