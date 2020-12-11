use std::fs;

fn main() {
    let program: Vec<usize> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let result: usize = find_first_error(&program, 25);
    println!("part 1: {}", result);
    let result_2: usize = find_weakness(&program, result);
    println!("part 2: {}", result_2);
}

fn find_first_error(input: &Vec<usize>, preamble_size: usize) -> usize {
    for window in input.windows(preamble_size + 1) {
        if !check_sum(window[..preamble_size].to_vec(), window[preamble_size]) {
            return window[preamble_size];
        }
    };
    0
}

fn check_sum(slice: Vec<usize>, sum: usize) -> bool {
    for (i, x) in slice.iter().enumerate() {
        for y in slice[i+1..].iter() {
            if x + y == sum {
                return true;
            }
        }
    };
    false
}

fn find_weakness(input: &Vec<usize>, weak_sum: usize) -> usize {
    for i in 2..input.iter().count() {
        for window in input.windows(i) {
            if window.iter().sum::<usize>() == weak_sum {
                let mut sorted: Vec<usize> = window.iter().cloned().collect();
                sorted.sort_unstable();
                return sorted[0] + sorted.last().unwrap();
            }
        }
    };
    0
}