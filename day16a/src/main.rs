use regex::Regex;

pub fn main() {
    let re = Regex::new(r#"^.*: (\d+)-(\d+) or (\d+)-(\d+)$"#).unwrap();
    let data = include_str!("../input.txt");

    let rules: Vec<(usize, usize, usize, usize)> = data
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|rule| {
            let cap = re.captures(rule).unwrap();
            (
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
                cap[3].parse::<usize>().unwrap(),
                cap[4].parse::<usize>().unwrap(),
            )
        })
        .collect();

    println!(
        "{}",
        data.lines()
            .skip(rules.len() + 1)
            .skip_while(|&t| t != "nearby tickets:")
            .skip(1)
            .flat_map(|ticket| {
                ticket
                    .split(',')
                    .map(|field| field.parse::<usize>().unwrap())
            })
            .filter(|&f| {
                !rules
                    .iter()
                    .any(|r| (f >= r.0 && f <= r.1) || (f >= r.2 && f <= r.3))
            })
            .sum::<usize>(),
    );
}
