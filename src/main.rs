extern crate lcg;

use lcg::LinearCongruentialGenerator;
use std::time::SystemTime;

fn main() {
    let mut rng = LinearCongruentialGenerator::new(seed_from_current_time());

    for _ in 0..24 {
        let [a, b, c, d, e, f, g, h] = rng.raw_next().to_be_bytes();

        println!(
            "{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
            a, b, c, d, e, f, g, h
        );
    }
}

fn seed_from_current_time() -> u64 {
    let now = SystemTime::now();

    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as u64,
        Err(_) => panic!("Somehow could not determine the time"),
    }
}
