use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();
    let mut count: u32 = 0;
    for line in input.chunks(3) {
        let minmax: Vec<usize> = line[0]
            .split("-")
            .map(|s| usize::from_str_radix(s, 10).expect("non-integer input"))
            .collect();
        let letter = line[1].chars().nth(0).unwrap();
        let first = line[2].chars().nth(minmax[0] - 1).unwrap();
        let second = line[2].chars().nth(minmax[1] - 1).unwrap();
        match (first == letter, second == letter) {
            (true, true) => (),
            (true, false) => count = count + 1,
            (false, true) => count = count + 1,
            (false, false) => (),
        };
    }
    println!("{}", count);
}
