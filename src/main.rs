use std::env;
use std::time::Instant;

fn eratos(limit: u32) -> Vec<u32> {
    let mut primes = vec![true; (limit + 1) as usize];
    primes[0] = false;
    primes[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if primes[p as usize] {
            let mut i = p * p;
            while i <= limit {
                primes[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }

    let mut result = Vec::new();
    for (num, is_prime) in primes.iter().enumerate() {
        if *is_prime {
            result.push(num as u32);
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_total: u32 = args[1].parse().unwrap();

    let start_time = Instant::now();

    let primes = eratos(num_total);
    //for prime in primes {
    //    println!("{}", prime);
    //}

    let end_time = start_time.elapsed();
    println!("Time taken: {:?}", end_time);
    println!("Number of primes: {}", primes.len());
}
