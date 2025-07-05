use num_bigint::BigUint;
use std::{cmp::max};

pub struct CollatzCaliculator {
    value: BigUint,
    length: BigUint,
    max_value: BigUint
}

impl CollatzCaliculator {
    pub fn new(initial_value: BigUint) -> Self {
        let length = BigUint::from(0u32);
        let max_value = initial_value.clone();
        Self {
            value: initial_value,
            length: length,
            max_value: max_value,
        }
    }

    pub fn simulate(&mut self) -> (BigUint, BigUint) {
        self.show();
        while self.value != BigUint::from(1u32) {
            self.next();
            self.show();
        }
        return self.get_log();
    }

    fn next(&mut self) {
        self.value = if self.value.clone() % BigUint::from(2u32) == BigUint::from(0u32) {
            self.value.clone() / BigUint::from(2u32)
        } else {
            self.value.clone() * BigUint::from(3u32) + BigUint::from(1u32)
        };
        self.length += BigUint::from(1u32);

        if self.max_value < self.value {
            self.max_value = self.value.clone();
        };
    }

    fn show(&self) {
        println!("{}: {}", self.length, self.value);
    }

    fn get_log(&self) -> (BigUint, BigUint) {
        return (self.length.clone(), self.max_value.clone());
    }
}
