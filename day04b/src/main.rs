#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};

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
                        .map(|i| i.split_once(':').unwrap())
                        .collect::<HashMap<&str, &str>>()
                })
                .flatten()
                .collect::<HashMap<&str, &str>>()
        })
        .filter(|passport| passport.iter().all(|(f, v)| validate_items(f, v)))
        .filter(|passport| required.iter().all(|item| passport.contains_key(item)))
        .count();

    println!("{}", valid);
}

fn validate_items(field: &str, value: &str) -> bool {
    match field {
        "byr" => {
            let value: usize = value.parse().unwrap();
            value >= 1920 && value <= 2002
        }
        "iyr" => {
            let value: usize = value.parse().unwrap();
            value >= 2010 && value <= 2020
        }
        "eyr" => {
            let value: usize = value.parse().unwrap();
            value >= 2020 && value <= 2030
        }
        "hgt" => {
            if value.ends_with("cm") && value.len() == 5 {
                let len: usize = value[0..3].parse().unwrap();
                len >= 150 && len <= 193
            } else if value.ends_with("in") && value.len() == 4 {
                let len: usize = value[0..2].parse().unwrap();
                len >= 59 && len <= 76
            } else {
                false
            }
        }
        "hcl" => value.len() == 7,
        "ecl" => {
            let one_of = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            one_of.iter().any(|v| &value == v)
        }
        "pid" => value.len() == 9,
        "cid" => true,
        _ => panic!("unknown field type"),
    }
}
