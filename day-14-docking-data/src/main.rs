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
            let key: usize = index_re.captures(&line[0]).unwrap()["i"].parse().unwrap();
            let mut value: String = format!("{:0>36b}", line[2].parse::<usize>().unwrap());
            for (i, c) in current_mask.chars().enumerate() {
                match c {
                    '1' => value.replace_range(i..i + 1, "1"),
                    '0' => value.replace_range(i..i + 1, "0"),
                    _ => (),
                }
            }
            memory.insert(key, value);
        }
    }
    for (k, v) in memory.iter() {
        println!("{}, {}", k, v);
    }
    let result: usize = memory
        .values()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .sum();

    //let result: usize = execute(&program);
    println!("part 1: {}", result);
    //let result_2: isize = execute_until_fixed(&program);
    //println!("part 2: {}", result_2);
}
