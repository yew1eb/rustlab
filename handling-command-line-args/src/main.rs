use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    if m != 0 {
        return gcd(m, n % m);
    } else {
        return n;
    }
}

fn main() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
