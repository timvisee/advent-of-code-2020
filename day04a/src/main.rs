use std::collections::HashSet;

const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    println!(
        "{}",
        std::fs::read_to_string("./input.txt")
            .unwrap()
            .split("\n\n")
            .map(|fields| fields
                .split_ascii_whitespace()
                .map(|field| field.split(':').next().unwrap())
                .collect::<HashSet<_>>())
            .filter(|passport| REQ_FIELDS.iter().all(|item| passport.contains(item)))
            .count(),
    );
}
