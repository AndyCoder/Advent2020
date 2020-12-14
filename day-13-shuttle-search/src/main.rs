//use std::fs;
fn main() {
    /*
    let input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();
    let timestamp: usize = input.as_slice()[0].parse().unwrap();
    let busses: Vec<String> = input.as_slice()[1].split(",")
        .map(|s| String::from(s))
        .collect();
    let result = part_2(&busses);
    println!("{}", result);
    */
    let mut ts: usize = 200_000_000_000_000;
    loop {
        if ts % 911 == 40
            && ts % 827 == 71
            && ts % 41 == 0
            && ts % 37 == 34
            && ts % 29 == 11
            && ts % 23 == 17
            && ts % 19 == 14
            && ts % 17 == 3
            && ts % 13 == 1
        {
            println!("{}", ts);
            break;
        }
        ts += 1;
    }
}

/*
fn part_2(busses: &[String]) -> usize {
    let mut timestamp: usize = 0;
    loop {

    }
    /*
    for ts in 100_000_000_000_000.. {
        let mut matches = true;
        println!("{}", ts);
        for (i, bus) in busses.iter().enumerate() {
            if bus != "x" {
                /*
                if !factor_cache.contains_key(&(ts+i)) {
                    factor_cache.insert(ts+i, sieve.factor(ts+i).unwrap().iter()
                    .map(|x| x.0).collect());
                }
                if !factor_cache.get(&(ts+i)).unwrap().contains(&bus.parse::<usize>().unwrap()) {
                    matches = false;
                    break;
                }
                */
                if (ts+i) % bus.parse::<usize>().unwrap() != 0 {
                    matches = false;
                    break;
                }
            }
        }
        if matches {
            timestamp = ts;
            break;
        }
    }
    */
    timestamp
}

fn part_1(ts: usize, busses: &[usize]) -> usize {
    let sieve = Sieve::new(10_000_000);
    let mut prime: usize = 0;
    let mut bus: usize = 0;
    'outer: for t in ts.. {
        let factors: Vec<usize> = sieve.factor(t).unwrap().iter()
            .map(|x| x.0).collect();
        for b in busses.iter() {
            if factors.contains(b) {
                prime = t.clone();
                bus = b.clone();
                break 'outer;
            }
        }
    }
    bus * (prime - ts)
}
*/
