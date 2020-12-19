#![feature(str_split_once)]

pub fn main() {
    let input = include_str!("../input.txt");
    let mut rules: Vec<(usize, Rule)> = input
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
    rules.sort_unstable_by_key(|r| r.0);
    let rules: Vec<Rule> = rules.into_iter().map(|r| r.1).collect();

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

fn matches(msg: &[u8], rules: &[Rule], rule: usize) -> Option<usize> {
    if msg.is_empty() {
        return None;
    }

    match &rules[rule] {
        Rule::Lit(c) if &msg[0] == c => Some(1),
        Rule::Lit(_) => None,
        Rule::Seq(r) => r
            .into_iter()
            .try_fold(0, |c, r| matches(&msg[c..], rules, *r).map(|n| n + c)),
        Rule::SeqOr(r, s) => r
            .into_iter()
            .try_fold(0, |c, r| matches(&msg[c..], rules, *r).map(|n| n + c))
            .or_else(|| {
                s.into_iter()
                    .try_fold(0, |c, r| matches(&msg[c..], rules, *r).map(|n| n + c))
            }),
    }
}

#[derive(Debug)]
enum Rule {
    Lit(u8),
    Seq(Vec<usize>),
    SeqOr(Vec<usize>, Vec<usize>),
}
