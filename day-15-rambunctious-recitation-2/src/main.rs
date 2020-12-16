use std::fs;

fn main() {
    let turns: Vec<usize> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut turn: usize = 0;
    let mut last_seen: Vec<usize> = vec![0; 30000000];
    let mut last: usize = turns.last().unwrap().clone();
    let len = turns.iter().count();
    for x in turns[0..len-1].iter() {
        turn += 1;
        last_seen[x.clone()] = turn;
    }
    turn += 1;
    loop {
        turn += 1;
        last = take_a_turn(&mut last_seen, turn, last);
        if turn >= 30000000 {
            break;
        }
    }
    println!("{}", last);
}

fn take_a_turn(last_seen: &mut Vec<usize>, turn: usize, last: usize) -> usize {
    let mut result: usize = 0;
    if last_seen[last] > 0 {
        result = turn - last_seen[last] - 1;
    }
    last_seen[last] = turn-1;
    result
}