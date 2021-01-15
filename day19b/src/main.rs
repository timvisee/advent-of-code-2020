#![feature(str_split_once)]

pub fn main() {
    let input = include_str!("../input.txt");
    let (rules, msgs) = input.split_once("\n\n").unwrap();
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
    let mut rules: Vec<Rule> = rules.into_iter().map(|r| r.1).collect();

    rules[8] = Rule::SeqOr(vec![42], vec![42, 8]);
    rules[11] = Rule::SeqOr(vec![42, 31], vec![42, 11, 31]);

    println!(
        "{}",
        msgs.lines()
            .filter(|m| matches(&m.as_bytes(), &rules, 0)
                // .map(|n| n == m.len())
                .iter()
                .any(|msg| msg.is_empty()))
            .count(),
    );
}

// TODO: list never larger than 8
fn matches<'a>(msg: &'a [u8], rules: &[Rule], rule: usize) -> Vec<&'a [u8]> {
    if msg.is_empty() {
        return vec![];
    }

    match &rules[rule] {
        Rule::Lit(c) if &msg[0] == c => vec![&msg[1..]],
        Rule::Lit(_) => vec![],
        Rule::Seq(a) => {
            a.iter().fold(vec![msg], |msgs, r| {
                // TODO: quit early if no msgs left, with try_fold?
                msgs.iter().flat_map(|m| matches(&m, rules, *r)).collect()
            })
        }
        Rule::SeqOr(a, b) => {
            let mut result = a.iter().fold(vec![msg], |msgs, r| {
                msgs.iter().flat_map(|m| matches(&m, rules, *r)).collect()
            });
            result.extend(b.iter().fold(vec![msg], |msgs, r| {
                msgs.iter().flat_map(|m| matches(&m, rules, *r)).collect()
            }));
            result
        }
    }
}

#[derive(Debug)]
enum Rule {
    Lit(u8),
    Seq(Vec<usize>),
    SeqOr(Vec<usize>, Vec<usize>),
}
