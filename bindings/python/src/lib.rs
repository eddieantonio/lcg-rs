use lcg::LinearCongruentialGenerator;
use pyo3::prelude::*;

/// LCG()
///
/// a simple linear congruential generator (psuedo-random number generator)
#[pyclass]
struct LCG(LinearCongruentialGenerator);

#[pymethods]
impl LCG {
    #[new]
    fn new(seed: u64) -> Self {
        LCG(LinearCongruentialGenerator::new(seed))
    }

    /// Returns a random int in the range of [-2**31, 2**31)
    fn next_signed(&mut self) -> i32 {
        self.0.next_i32()
    }

    /// Returns a random int in the range of [0, 2**32 - 1)
    fn next_unsigned(&mut self) -> u32 {
        self.0.next_u32()
    }
}

#[pymodule]
fn lcg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LCG>()?;
    Ok(())
}
