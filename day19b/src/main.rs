#![feature(str_split_once)]

fn main() {
    let (rules, msgs) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut rules: Vec<(usize, Rule)> = rules
        .lines()
        .map(|l| {
            let (n, rule) = l.split_once(": ").unwrap();
            (
                n.parse().unwrap(),
                if rule.starts_with('"') {
                    Rule::Lit(rule.chars().nth(1).unwrap() as u8)
                } else {
                    let parts: Vec<_> = rule.splitn(2, '|').collect();
                    let a = parts[0]
                        .split_terminator(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse().unwrap())
                        .collect();

                    if parts.len() == 1 {
                        Rule::Seq(a)
                    } else {
                        let b = parts[1]
                            .split_terminator(' ')
                            .filter(|n| !n.is_empty())
                            .map(|n| n.parse().unwrap())
                            .collect();
                        Rule::SeqOr(a, b)
                    }
                },
            )
        })
        .collect();
    rules.sort_unstable_by_key(|r| r.0);
    let rules: Vec<_> = rules.into_iter().map(|r| r.1).collect();

    // rules[8] = Rule::SeqOr(vec![42], vec![42, 8]);
    // rules[11] = Rule::SeqOr(vec![42, 31], vec![42, 11, 31]);

    println!(
        "{}",
        msgs.lines()
            .filter(|msg| matches_42(msg.as_bytes(), &rules))
            .count(),
    );
}

enum Rule {
    Lit(u8),
    Seq(Vec<usize>),
    SeqOr(Vec<usize>, Vec<usize>),
}

fn matches_42(msg: &[u8], rules: &[Rule]) -> bool {
    (0..)
        .try_fold(msg, |msg, depth| match matches(msg, 42, rules) {
            Some(msg) if matches_31(depth, msg, rules) => Err(true),
            Some(msg) => Ok(msg),
            None => Err(false),
        })
        .err()
        .unwrap()
}

fn matches_31(depth: usize, msg: &[u8], rules: &[Rule]) -> bool {
    (0..depth)
        .try_fold(msg, |msg, _| match matches(msg, 31, rules) {
            Some(msg) if msg.is_empty() => Err(true),
            Some(msg) => Ok(msg),
            None => Err(false),
        })
        .err()
        .unwrap_or(false)
}

fn matches<'a>(msg: &'a [u8], rule: usize, rules: &[Rule]) -> Option<&'a [u8]> {
    match &rules[rule] {
        Rule::Lit(_) if msg.is_empty() => None,
        Rule::Lit(c) if &msg[0] == c => Some(&msg[1..]),
        Rule::Lit(_) => None,
        Rule::Seq(a) => a.into_iter().try_fold(msg, |m, &r| matches(m, r, rules)),
        Rule::SeqOr(a, b) => a
            .into_iter()
            .try_fold(msg, |m, &r| matches(m, r, rules))
            .or_else(|| b.into_iter().try_fold(msg, |m, &r| matches(m, r, rules))),
    }
}
