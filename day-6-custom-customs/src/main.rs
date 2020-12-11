#![feature(iterator_fold_self)]

use std::collections::HashSet;
use std::fs;

fn main() {
    let input: Vec<usize> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n\n")
        .map(|s| count_chars(s))
        .collect();
    let result: usize = input.iter().sum();
    println!("{}", result);
}

fn count_chars(input: &str) -> usize {
    input
        .split_whitespace()
        .map(|s| char_set(s))
        .fold_first(|a, b| b.intersection(&a).cloned().collect())
        .unwrap()
        .iter()
        .count()
}

fn char_set(input: &str) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for c in input.chars() {
        set.insert(c);
    }
    set
}
