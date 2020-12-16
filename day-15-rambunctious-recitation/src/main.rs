use std::fs;

fn main() {
    let mut turns: Vec<usize> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", turns);
    let mut turn: usize = turns.iter().count();
    loop {
        println!("{}", turn);
        turns = take_a_turn(turns);
        turn += 1;
        if turn >= 30000000 {
            break;
        }
    }
    for (i, x) in turns.iter().enumerate() {
        println!("{}: {}", i+1, x);
    }
}

fn take_a_turn(turns: Vec<usize>) -> Vec<usize> {
    let mut next_turn = turns.to_vec();
    let last = turns.last().unwrap().clone();
    let len = turns.iter().count();
    if !turns[0..len-1].contains(&last) {
        next_turn.push(0)
    } else {
        for (i, x) in turns[..len-1].iter().enumerate().rev() {
            if x.clone() == last {
                next_turn.push(len - (i+1));
                break;
            }
        };
    }
    next_turn
}