use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| String::from(s))
        .collect();
    let busses: Vec<String> = input.as_slice()[1].split(",")
        .map(|s| String::from(s))
        .collect();
    let mut timestamp: u128 = busses[0].parse().unwrap();
    let mut lcm: u128 = busses[0].parse().unwrap();
    for (i, x) in busses[1..].iter().enumerate() {
        println!("i = {}, x = {}, timestamp = {}, lcm = {}", i, x, timestamp, lcm);
        if x != "x" {
            loop {
                let modulo = x.parse::<u128>().unwrap();
                if timestamp % modulo == modulo - ((i as u128 +1)%modulo) {
                    println!("{} % {} == {} - (({}+1) % {}), {}", timestamp, modulo, modulo, i, modulo, ((i as u128 +1)%modulo));
                    lcm = lcm * modulo;
                    break;
                }
                timestamp += lcm;
            }
        }
    }
    println!("{}", timestamp);
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
