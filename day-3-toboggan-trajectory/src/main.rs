use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();
    let count_11 = count_trees_on_slope(&input, 1, 1);
    let count_31 = count_trees_on_slope(&input, 3, 1);
    let count_51 = count_trees_on_slope(&input, 5, 1);
    let count_71 = count_trees_on_slope(&input, 7, 1);
    let count_12 = count_trees_on_slope(&input, 1, 2);
    println!(
        "{} x {} x {} x {} x {} = {}",
        count_11,
        count_31,
        count_51,
        count_71,
        count_12,
        count_11 * count_31 * count_51 * count_71 * count_12
    );
}

fn count_trees_on_slope(input: &Vec<String>, right: usize, down: usize) -> usize {
    let mut count: usize = 0;
    let mut pos_y: usize = 0;
    let width = input[0].chars().count();
    for chunk in input.chunks(down) {
        if pos_y >= width {
            pos_y = pos_y - width;
        }
        if chunk[0].chars().nth(pos_y).unwrap() == '#' {
            count = count + 1;
        }
        pos_y = pos_y + right;
    }
    count
}
