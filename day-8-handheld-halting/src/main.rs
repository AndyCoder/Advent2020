use std::collections::HashSet;
use std::fs;

fn main() {
    let program: Vec<Instruction> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| split_line(s))
        .collect();
    let result: isize = execute(&program);
    println!("part 1: {}", result);
    let result_2: isize = execute_until_fixed(&program);
    println!("part 2: {}", result_2);
}

struct Instruction {
    op: Operation,
    arg: isize,
}

enum Operation {
    Acc,
    Jmp,
    Nop,
}

fn split_line(input: &str) -> Instruction {
    let first_split: Vec<String> = input.split_whitespace().map(|s| s.to_owned()).collect();
    Instruction {
        op: match first_split[0].as_str() {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => Operation::Nop,
        },
        arg: first_split[1].parse::<isize>().unwrap(),
    }
}

fn execute(program: &Vec<Instruction>) -> isize {
    let mut indices: HashSet<usize> = HashSet::with_capacity(program.iter().count());
    let mut current_index = 0;
    let mut accumulator = 0;
    loop {
        if indices.contains(&current_index) {
            break;
        } else {
            indices.insert(current_index);
        };
        match program[current_index].op {
            Operation::Acc => {
                accumulator = accumulator + program[current_index].arg;
                current_index = current_index + 1;
            }
            Operation::Jmp => {
                current_index = (current_index as isize + program[current_index].arg) as usize;
            }
            Operation::Nop => {
                current_index = current_index + 1;
            }
        };
    }
    accumulator
}

fn execute_until_fixed(program: &Vec<Instruction>) -> isize {
    let mut modified_indices: HashSet<usize> = HashSet::with_capacity(program.iter().count());
    'outer: loop {
        let mut indices: HashSet<usize> = HashSet::with_capacity(program.iter().count());
        let mut modified_this_iter = false;
        let mut accumulator = 0;
        let mut current_index = 0;
        'inner: loop {
            if indices.contains(&current_index) {
                break 'inner;
            } else if current_index >= program.iter().count() {
                break 'outer accumulator;
            } else {
                indices.insert(current_index);
            };
            match program[current_index].op {
                Operation::Acc => {
                    accumulator = accumulator + program[current_index].arg;
                    current_index = current_index + 1;
                }
                Operation::Jmp => {
                    if modified_this_iter || modified_indices.contains(&current_index) {
                        current_index =
                            (current_index as isize + program[current_index].arg) as usize;
                    } else {
                        modified_indices.insert(current_index);
                        modified_this_iter = true;
                        current_index = current_index + 1;
                    }
                }
                Operation::Nop => {
                    if modified_this_iter || modified_indices.contains(&current_index) {
                        current_index = current_index + 1;
                    } else {
                        modified_indices.insert(current_index);
                        modified_this_iter = true;
                        current_index =
                            (current_index as isize + program[current_index].arg) as usize;
                    }
                }
            };
        }
    }
}
