use std::collections::HashSet;

const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .lines()
            .scan(Vec::new(), |ref mut buf, line| {
                Some(if !line.is_empty() {
                    buf.push(line);
                    None
                } else {
                    let map = buf
                        .into_iter()
                        .flat_map(|l| l.split(' ').map(|f| f.split(':').next().unwrap()))
                        .collect::<HashSet<_>>();
                    buf.clear();
                    Some(map)
                })
            })
            .filter_map(|f| f)
            .filter(|passport| REQ_FIELDS.iter().all(|item| passport.contains(item)))
            .count(),
    );
}
