use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^([a-z ]+) bags contain (.*)$"#).unwrap();
    static ref RE_CONT: Regex = Regex::new(r#"\d ([a-z ]+) b"#).unwrap();
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let rules: Vec<_> = data.lines().map(parse_bag).collect();

    let (mut bags, mut cursor) = (vec!["shiny gold"], 0);

    while let Some(target) = bags.get(cursor) {
        let extra = rules
            .iter()
            .filter(|(color, cont)| cont.contains(target) && !bags.contains(color))
            .map(|(color, _)| *color)
            .collect::<Vec<_>>();
        bags.extend_from_slice(&extra);
        cursor += 1;
    }

    println!("{}", bags.len() - 1);
}

/// Parse bag ruleset.
#[inline(always)]
fn parse_bag<'a>(rule: &'a str) -> (&'a str, Vec<&str>) {
    let captures = RE_RULE.captures(rule).unwrap();
    (
        captures.get(1).unwrap().as_str(),
        RE_CONT
            .captures_iter(captures.get(2).unwrap().as_str())
            .map(|cond| cond.get(1).unwrap().as_str())
            .collect(),
    )
}
