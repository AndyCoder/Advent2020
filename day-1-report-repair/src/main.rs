use std::fs;

fn main() {
    let mut ascending: Vec<u32> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| u32::from_str_radix(s, 10).expect("non-integer input"))
        .collect();
    ascending.sort_unstable();
    for (i, x) in ascending.iter().enumerate() {
        for (j, y) in ascending.as_slice()[i + 1..].iter().enumerate() {
            for z in ascending.as_slice()[i + j + 2..].iter().rev() {
                if x + y + z < 2020 {
                    break;
                } else if x + y + z == 2020 {
                    println!("{} + {} + {} = {}", x, y, z, 2020);
                    std::process::exit(0);
                }
            }
        }
    }
}
