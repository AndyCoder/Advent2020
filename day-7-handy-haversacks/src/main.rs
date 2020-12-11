#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let rules: HashMap<String, Vec<QuantityOfBags>> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| split_line(s))
        .collect();
    let result: usize = rules
        .keys()
        .map(|k| match sgb_is_reachable(&rules, k) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("part 1: {}", result);
    let result_2: usize = count_bags(&rules, "shiny gold bags");
    println!("part 2: {}", result_2);
}

struct QuantityOfBags {
    quantity: usize,
    kind: String,
}

fn split_line(input: &str) -> (String, Vec<QuantityOfBags>) {
    let first_split: Vec<String> = input.split(" contain ").map(|s| s.to_owned()).collect();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b\d+[ \w]+bag").unwrap();
    }
    let values: Vec<QuantityOfBags> = RE
        .find_iter(&first_split[1])
        .map(|s| QuantityOfBags {
            quantity: usize::from_str_radix(s.as_str().get(0..1).unwrap(), 10).unwrap(),
            kind: s.as_str().get(2..).unwrap().to_owned() + "s",
        })
        .collect();
    (first_split[0].clone(), values)
}

fn sgb_is_reachable(rules: &HashMap<String, Vec<QuantityOfBags>>, key: &str) -> bool {
    match rules.get(key) {
        Some(val) => val
            .iter()
            .any(|x| x.kind == "shiny gold bags" || sgb_is_reachable(rules, &x.kind)),
        None => false,
    }
}

fn count_bags(rules: &HashMap<String, Vec<QuantityOfBags>>, key: &str) -> usize {
    match rules.get(key) {
        Some(val) => val
            .iter()
            .map(|x| x.quantity + x.quantity * count_bags(rules, &x.kind))
            .sum(),
        None => 1,
    }
}
