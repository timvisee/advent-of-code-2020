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
                let (p1, p2): (usize, usize) = (p[1].parse().unwrap(), p[2].parse().unwrap());
                let c = p[3].as_bytes()[0];
                let s = p[4].as_bytes();
                (s[p1 - 1] == c) != (s[p2 - 1] == c)
            })
            .count(),
    );
}
