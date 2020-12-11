use std::collections::HashMap;
use std::fs;

fn main() {
    let mut adapters: Vec<usize> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    adapters.sort_unstable();
    adapters.insert(0, 0);
    adapters.push(adapters.as_slice().last().copied().unwrap() + 3);
    let result: usize = part_1(&adapters);
    println!("part 1: {}", result);
    let mut memos = HashMap::new();
    let result_2: usize = part_2(&mut memos, &adapters, 0);
    println!("part 2: {}", result_2);
}

fn part_1(input: &[usize]) -> usize {
    let mut ones = 0;
    let mut threes = 0;
    for window in input.windows(2) {
        if window[1] == window[0] + 1 {
            ones = ones + 1;
        } else if window[1] == window[0] + 3 {
            threes = threes + 1;
        } else if window[1] == window[0] + 2 {
            println!("{:?}", window);
        }
    }
    ones * threes
}

fn part_2(memos: &mut HashMap<usize, usize>, input: &[usize], i: usize) -> usize {
    if !input.contains(&i) {
        0
    } else if i == input.last().copied().unwrap() {
        1
    } else {
        if !memos.contains_key(&i) {
            let x = part_2(memos, input, i + 1)
                + part_2(memos, input, i + 2)
                + part_2(memos, input, i + 3);
            memos.insert(i, x);
        }
        memos.get(&i).unwrap().clone()
    }
}
