use regex::Regex;

pub fn main() {
    let re = Regex::new(r#"^(\d+)-(\d+) (.): (.+)$"#).unwrap();
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|p| re.captures(p).unwrap())
            .filter(|p| {
                let len = p[4].matches(&p[3]).count();
                len >= p[1].parse().unwrap() && len <= p[2].parse().unwrap()
            })
            .count(),
    );
}
