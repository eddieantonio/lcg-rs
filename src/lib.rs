use std::num::Wrapping;

#[derive(Copy, Clone)]
pub struct LinearCongruentialGenerator {
    seed: u64,
}

impl LinearCongruentialGenerator {
    // Stealing values from the POXIX:2018 drand48(3)
    // https://pubs.opengroup.org/onlinepubs/9699919799/functions/drand48.html
    const A: Wrapping<u64> = Wrapping(0x5DEECE66Du64);
    const C: Wrapping<u64> = Wrapping(0x00000000Bu64);
    const M: u64 = 2_u64.pow(48);

    pub fn new(seed: u64) -> Self {
        LinearCongruentialGenerator { seed }
    }

    pub fn next_u48(&mut self) -> u64 {
        let x = Wrapping(self.seed);

        self.seed = (Self::A * x + Self::C).0 % Self::M;

        self.seed
    }
}

#[cfg(test)]
mod tests {
    use crate::LinearCongruentialGenerator;
    use std::time::SystemTime;

    #[test]
    fn it_generates_random_numbers() {
        let now = SystemTime::now();
        let initial_value = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos() as u64,
            Err(_) => panic!("Somehow could not determine the time"),
        };

        let mut rng = LinearCongruentialGenerator::new(initial_value);
        let first_value = rng.next_u48();
        let second_value = rng.next_u48();
        assert_ne!(initial_value, first_value);
        assert_ne!(first_value, second_value);
    }
}
