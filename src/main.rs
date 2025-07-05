use num_bigint::BigUint;

mod caliculator;
use caliculator::{CollatzCaliculator};

fn main() {
    let initial_value = BigUint::from(837799u32);
    let mut caliculator = CollatzCaliculator::new_with_debug(initial_value);
    let (count, max) = caliculator.simulate();
    println!("{} {}", count, max);
}
