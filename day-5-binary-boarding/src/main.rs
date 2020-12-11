use std::fs;

fn main() {
    let mut input: Vec<usize> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| pass_id(s))
        .collect();
    input.sort_unstable();
    for window in input.as_slice().windows(2) {
        if window[1] - window[0] == 2 {
            println!("{} < {} < {}", window[0], window[0] + 1, window[1]);
        }
    }
}

fn pass_id(input: &str) -> usize {
    let row = chars_to_base_2(&input[0..7], 'F', 'B');
    let col = chars_to_base_2(&input[6..], 'L', 'R');
    row * 8 + col
}

fn chars_to_base_2(s: &str, c_zero: char, c_one: char) -> usize {
    let string: String = s
        .chars()
        .map(|c| match c {
            c if c == c_zero => '0',
            c if c == c_one => '1',
            _ => '0',
        })
        .collect();
    usize::from_str_radix(&string, 2).unwrap()
}
