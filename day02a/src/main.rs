use regex::Regex;

fn main() {
    let re = Regex::new(r#"^(\d+)-(\d+) (.): (.+)$"#).unwrap();
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .map(|p| re.captures(p).unwrap())
            .filter(|p| {
                let len = p[4].matches(&p[3]).count();
                len >= p[1].parse().unwrap() && len <= p[2].parse().unwrap()
            })
            .count(),
    );
}
