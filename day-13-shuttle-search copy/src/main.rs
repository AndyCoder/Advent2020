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
    let mut ts: usize = 1;
    loop {
        if ts % 41 == 0 && ts % 37 == 35 {
            break;
        }
        ts += 1;
    }
    loop {
        if ts % 911 == 41 {
            break;
        }
        ts += 1_517;
    }
    loop {
        if ts % 13 == 2 {
            break;
        }
        ts += 1_381_987;
    }
    loop {
        if ts % 17 == 4 {
            break;
        }
        ts += 17_965_831;
    }
    loop {
        if ts % 23 == 18 {
            break;
        }
        ts += 305_419_127;
    }
    loop {
        if ts % 29 == 12 {
            break;
        }
        ts += 7_024_639_921;
    }
    loop {
        if ts % 827 == 72 {
            break;
        }
        ts += 203_714_557_709;
    }
    loop {
        if ts % 19 == 15 {
            break;
        }
        ts += 168_471_939_225_343;
    }
    println!("{}", ts);
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
