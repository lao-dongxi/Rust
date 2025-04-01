use std::{env, str::FromStr};
fn main() {
    //
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("err parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage:gcd NUMBER...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for i in &numbers[1..] {
        d = gcd(d, *i);
    }
    println!("the greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while n != 0 {
        let temp = m % n;
        m = n;
        n = temp;
        // println!("{}",n);
    }
    m
}
