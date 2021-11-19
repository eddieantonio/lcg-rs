extern crate lcg;

use lcg::LinearCongruentialGenerator;
use std::time::SystemTime;

const ITERATIONS: usize = 1_000_000_000;

fn main() {
    let mut rng = LinearCongruentialGenerator::new(seed_from_current_time());

    let mut n: u32 = 0;
    for _ in 0..ITERATIONS {
        n = rng.next_u32();
    }

    println!("{}", n);
}

fn seed_from_current_time() -> u64 {
    let now = SystemTime::now();

    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as u64,
        Err(_) => panic!("Somehow could not determine the time"),
    }
}
