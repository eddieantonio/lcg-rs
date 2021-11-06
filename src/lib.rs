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

    const LOWER_ORDER_SEED: u16 = 0x330E;

    pub fn new(seed: u64) -> Self {
        let seed = (seed & 0xFFFFFFFF) << 16 | Self::LOWER_ORDER_SEED as u64;

        LinearCongruentialGenerator { seed }
    }

    pub fn raw_next(&mut self) -> u64 {
        let x = Wrapping(self.seed);

        self.seed = (Self::A * x + Self::C).0 % Self::M;

        self.seed
    }

    pub fn next_u32(&mut self) -> u32 {
        let high_32bit_mask = ((!0u32) as u64) << 16;
        let raw_output = self.raw_next();

        ((raw_output & high_32bit_mask) >> 16) as u32
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    pub fn next_uniform(&mut self) -> f32 {
        let value = self.next_uniform_cast_and_divide();

        assert!(!value.is_nan());
        assert!(value.is_finite());
        assert!(value.is_sign_positive());
        assert!(value.is_finite());
        assert!(value >= 0.0);
        assert!(value < 1.0);

        value
    }

    fn next_uniform_cast_and_divide(&mut self) -> f32 {
        (self.next_u32() as f64 / u32::MAX as f64) as f32
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

        let first_value = rng.raw_next();
        let second_value = rng.raw_next();
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

        let first_value = rng.raw_next();
        assert_eq!(0, first_value & upper_bit_mask);
        assert_ne!(0, first_value & output_bit_mask);

        let second_value = rng.raw_next();
        assert_eq!(0, second_value & upper_bit_mask);
        assert_ne!(0, second_value & output_bit_mask);
    }

    #[test]
    fn it_generates_negative_numbers() {
        let max_tries = 32;

        let initial_value = seed_from_current_time();
        let mut rng = LinearCongruentialGenerator::new(initial_value);

        let mut nonnegatives_generated = 0;
        let mut negatives_generated = 0;
        for _ in 0..max_tries {
            let value = rng.next_i32();
            if value < 0 {
                negatives_generated += 1;
            } else {
                nonnegatives_generated += 1;
            }
        }

        assert!(negatives_generated > 0);
        assert!(nonnegatives_generated > 0);
    }

    #[test]
    fn it_is_decently_random() {
        // Uses the Monty Carlo estimation of π to determine if we're "random enough"
        // A true uniform random distribution of numbers will eventually eventually cover enough
        // random points that will eventuall converge to π

        // It's okay if we're off by this much:
        let tolerance = 0.025; // 2.5% relative error. Better than most psych papers 😏
        let rounds: usize = 10_000;

        let initial_value = seed_from_current_time();
        let mut rng = LinearCongruentialGenerator::new(initial_value);

        let mut samples_within_circle = 0;
        for _ in 0..rounds {
            let x = rng.next_uniform();
            let y = rng.next_uniform();

            let inside_circle = (x * x + y * y) <= 1.0;
            if inside_circle {
                samples_within_circle += 1;
            }
        }

        let std_pi = std::f32::consts::PI;
        let pi_estimate = 4.0 * samples_within_circle as f32 / rounds as f32;

        let absolute_error = (pi_estimate - std_pi).abs();
        let relative_error = absolute_error / std_pi;

        assert!(relative_error < tolerance);
    }

    fn seed_from_current_time() -> u64 {
        let now = SystemTime::now();

        match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos() as u64,
            Err(_) => panic!("Somehow could not determine the time"),
        }
    }
}
