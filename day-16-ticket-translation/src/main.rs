use std::fs;
use regex::Regex;
use regex::RegexSet;
use std::collections::HashMap;

fn main() {
    let sections: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n\n")
        .map(|s| String::from(s))
        .collect();
    let rules = parse_rules(&sections[0]);
}

fn parse_rules(input: &String) -> HashMap<String, Box<dyn Fn(usize)->bool>> {
    let mut rules_map = HashMap::new();
    rules_map
}

fn validate_passport(input: &str) -> bool {
    let req_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    if req_fields.iter().any(|f| input.matches(f).count() == 0) {
        return false;
    }
    let req_regex = RegexSet::new(&[
        r"\bbyr:(19[23456789]\d|2000|2001|2002)\b",
        r"\biyr:(201\d|2020)\b",
        r"\beyr:(202\d|2030)\b",
        r"\bhgt:(1[5678]\dcm|19[0123]cm|59in|6\din|7[0123456]in)\b",
        r"\bhcl:#[a-f0-9]{6}\b",
        r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b",
        r"\bpid:\d{9}\b",
    ])
    .unwrap();
    req_regex.matches(input).iter().count() == 7
}
