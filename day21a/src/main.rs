#![feature(str_split_once, map_into_keys_values)]

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

fn main() {
    let food: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|food| {
            let (ingr, aller) = food.split_once(" (contains ").unwrap();
            (
                ingr.split(' ').collect(),
                aller.trim_end_matches(')').split(", ").collect(),
            )
        })
        .collect();

    let good_ingr = take_good_ingr(&food, take_bad_ingr(&food).into_keys().collect());
    println!(
        "{}",
        food.iter()
            .map(|f| f.0.iter().filter(|i| good_ingr.contains(*i)).count())
            .sum::<usize>()
    );
}

fn take_bad_ingr<'a>(food: &[(HashSet<&'a str>, HashSet<&'a str>)]) -> HashMap<&'a str, &'a str> {
    let mut rev: HashMap<&str, HashSet<_>> = food
        .into_iter()
        .flat_map(|food| food.1.iter().map(move |aller| (food, aller)))
        .fold(HashMap::new(), |mut map, (food, aller)| {
            map.entry(aller)
                .and_modify(|ingr| *ingr = ingr.intersection(&food.0).copied().collect())
                .or_insert_with(|| food.0.clone());
            map
        });

    // Keep eliminating allergens with 1 ingredient from list
    let covered = RefCell::new(HashSet::new());
    loop {
        let covering: HashSet<_> = rev
            .iter()
            .filter(|(aller, ings)| ings.len() == 1 && !covered.borrow().contains(*aller))
            .map(|(aller, ings)| {
                covered.borrow_mut().insert(*aller);
                *ings.iter().next().unwrap()
            })
            .collect();
        match covering.is_empty() {
            false => rev
                .values_mut()
                .filter(|ings| ings.len() != 1)
                .for_each(|ings| *ings = ings.difference(&covering).copied().collect()),
            true => break,
        }
    }

    rev.into_iter()
        .map(|(aller, ingr)| (ingr.into_iter().next().unwrap(), aller))
        .collect()
}

fn take_good_ingr<'a>(
    food: &[(HashSet<&'a str>, HashSet<&'a str>)],
    bad_ingr: Vec<&str>,
) -> HashSet<&'a str> {
    food.into_iter()
        .flat_map(|food| food.0.iter().copied())
        .filter(|ingr| !bad_ingr.contains(ingr))
        .collect()
}
