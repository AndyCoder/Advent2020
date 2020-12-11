use regex::RegexSet;
use std::fs;

fn main() {
    let count = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n\n")
        .map(|s| validate_passport(s))
        .fold(0, |acc, x| if x { acc + 1 } else { acc });
    println!("{}", count)
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
