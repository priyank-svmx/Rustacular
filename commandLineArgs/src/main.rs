use std::env;
use std::str::FromStr;
fn main() {
    println!("Hello, world!");
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing the argument"));
    }

    if numbers.len() == 0 {
        eprintln!("usage: gcd NUMBER...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("the greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut d: u64, mut n: u64) -> u64 {
    return d * n;
}
