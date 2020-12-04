#![feature(str_split_once)]

use std::collections::HashMap;

const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

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
                        .flat_map(|l| l.split(' ').map(|f| f.split_once(':').unwrap()))
                        .collect::<HashMap<_, _>>();
                    buf.clear();
                    Some(map)
                })
            })
            .filter_map(|f| f)
            .filter(|passport| REQ_FIELDS.iter().all(|item| passport.contains_key(item)))
            .filter(|passport| passport.iter().all(|(f, v)| validate_field(f, v)))
            .count(),
    );
}

/// Validate a password field value.
fn validate_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => value.parse::<usize>().unwrap().wrapping_sub(1920) <= 82,
        "iyr" => value.parse::<usize>().unwrap().wrapping_sub(2010) <= 10,
        "eyr" => value.parse::<usize>().unwrap().wrapping_sub(2020) <= 10,
        "hgt" => {
            if value.ends_with("cm") && value.len() == 5 {
                value[0..3].parse::<usize>().unwrap().wrapping_sub(150) <= 43
            } else if value.ends_with("in") && value.len() == 4 {
                value[0..2].parse::<usize>().unwrap().wrapping_sub(59) <= 27
            } else {
                false
            }
        }
        "hcl" => value.len() == 7,
        "ecl" => EYE_COLORS.iter().any(|v| v == &value),
        "pid" => value.len() == 9,
        "cid" => true,
        _ => panic!("unknown field type"),
    }
}
