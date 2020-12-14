use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let program: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();
    let mut memory: HashMap<usize, String> = HashMap::new();
    let mut current_mask = String::from("");
    let index_re = Regex::new(r"mem\[(?P<i>\d+)\]").unwrap();
    for line in program.chunks(3) {
        if line[0] == "mask" {
            current_mask = line[2].to_string();
        } else {
            let key: String = format!(
                "{:0>36b}",
                index_re.captures(&line[0]).unwrap()["i"]
                    .parse::<usize>()
                    .unwrap()
            );
            let value: String = format!("{:0>36b}", line[2].parse::<usize>().unwrap());
            let keys = generate_range(key, current_mask.clone(), 0);
            for k in keys.iter() {
                memory.insert(usize::from_str_radix(&k, 2).unwrap(), value.clone());
            }
        }
    }
    for (k, v) in memory.iter() {
        println!("{}, {}", k, v);
    }
    let result: usize = memory
        .values()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .sum();
    println!("part 2: {}", result);
}

fn generate_range(input: String, mask: String, index: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if index < input.chars().count() {
        match mask.chars().nth(index).unwrap() {
            'X' => {
                let mut one = input.clone();
                let mut zero = input.clone();
                one.replace_range(index..index + 1, "1");
                zero.replace_range(index..index + 1, "0");
                result.append(&mut generate_range(one, mask.clone(), index + 1));
                result.append(&mut generate_range(zero, mask.clone(), index + 1));
            }
            '1' => {
                let mut one = input.clone();
                one.replace_range(index..index + 1, "1");
                result.append(&mut generate_range(one, mask.clone(), index + 1));
            }
            _ => result.append(&mut generate_range(input, mask, index + 1)),
        }
    } else {
        return vec![input];
    }
    result
}
