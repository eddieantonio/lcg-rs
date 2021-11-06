use std::num::Wrapping;

#[derive(Copy, Clone)]
pub struct LinearCongruentialGenerator {
    seed: u64,
}

impl LinearCongruentialGenerator {
    pub const OUTPUT_BITS: u32 = 48;

    // Stealing values from the POXIX:2018 drand48(3)
    // https://pubs.opengroup.org/onlinepubs/9699919799/functions/drand48.html
    const A: Wrapping<u64> = Wrapping(0x5DEECE66Du64);
    const C: Wrapping<u64> = Wrapping(0x00000000Bu64);
    const M: u64 = 2_u64.pow(Self::OUTPUT_BITS);

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
        let initial_value = seed_from_current_time();
        let mut rng = LinearCongruentialGenerator::new(initial_value);

        let first_value = rng.next_u48();
        let second_value = rng.next_u48();
        assert_ne!(initial_value, first_value);
        assert_ne!(first_value, second_value);
    }

    #[test]
    fn it_produces_48_bits_of_output() {
        let initial_value = seed_from_current_time();
        let mut rng = LinearCongruentialGenerator::new(initial_value);

        // Helps extract bits from the generator
        //  Output: 0000 1010 1011
        //    Mask: 0000 1111 1111
        //   Upper: 1111 0000 0000
        let output_bit_mask = 2_u64.pow(LinearCongruentialGenerator::OUTPUT_BITS) - 1;
        let upper_bit_mask = !output_bit_mask;

        let first_value = rng.next_u48();
        assert_eq!(0, first_value & upper_bit_mask);
        assert_ne!(0, first_value & output_bit_mask);

        let second_value = rng.next_u48();
        assert_eq!(0, second_value & upper_bit_mask);
        assert_ne!(0, second_value & output_bit_mask);
    }

    fn seed_from_current_time() -> u64 {
        let now = SystemTime::now();

        match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos() as u64,
            Err(_) => panic!("Somehow could not determine the time"),
        }
    }
}
