use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let valid = data
        .lines()
        .fold(
            (Vec::new(), Vec::with_capacity(8)),
            |(mut passports, mut acc), line| {
                if line.is_empty() {
                    passports.push(acc);
                    acc = Vec::with_capacity(8);
                } else {
                    acc.push(line);
                }

                (passports, acc)
            },
        )
        .0
        .into_iter()
        .map(|passport| {
            passport
                .into_iter()
                .map(|l| {
                    l.split(' ')
                        .map(|i| i.split(':').next().unwrap())
                        .collect::<HashSet<_>>()
                })
                .flatten()
                .collect::<HashSet<_>>()
        })
        .filter(|passport| required.iter().all(|item| passport.contains(item)))
        .count();

    println!("{}", valid);
}
