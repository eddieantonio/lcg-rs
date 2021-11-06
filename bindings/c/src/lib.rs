extern crate lcg;
extern crate libc;

use lcg::LinearCongruentialGenerator as LCG;

#[no_mangle]
pub extern "C" fn lcg_init(seed: u64) -> LCG {
    LCG::new(seed)
}

#[no_mangle]
pub extern "C" fn lcg_rand(rng: *mut LCG) -> u32 {
    unsafe {
        match rng.as_mut() {
            Some(r) => r.next_u32(),
            None => panic!("passed null to lcg_rand"),
        }
    }
}
