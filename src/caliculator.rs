use num_bigint::BigUint;

pub struct CollatzCaliculator {
    value: BigUint,
    length: BigUint,
    max_value: BigUint,
    show_steps: bool,
}

impl CollatzCaliculator {
    pub fn new(initial_value: BigUint) -> Self {
        let length = BigUint::from(0u32);
        let max_value = initial_value.clone();
        Self {
            value: initial_value,
            length: length,
            max_value: max_value,
            show_steps: false,
        }
    }

    pub fn new_with_debug(initial_value: BigUint) -> Self {
        let length = BigUint::from(0u32);
        let max_value = initial_value.clone();
        Self {
            value: initial_value,
            length: length,
            max_value: max_value,
            show_steps: true,
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
        if self.show_steps {
            println!("{}: {}", self.length, self.value);
        }
    }

    fn get_log(&self) -> (BigUint, BigUint) {
        return (self.length.clone(), self.max_value.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 親モジュールの関数を使うため

    #[test]
    fn test1() {
        let mut c = CollatzCaliculator::new(BigUint::from(1u32));
        let (a, b) = c.simulate();
        assert_eq!(a, BigUint::from(0u32));
        assert_eq!(b, BigUint::from(1u32));
    }

    #[test]
    fn test2() {
        let mut c = CollatzCaliculator::new(BigUint::from(27u32));
        let (a, b) = c.simulate();
        assert_eq!(a, BigUint::from(111u32));
        assert_eq!(b, BigUint::from(9232u32));
    }
}
