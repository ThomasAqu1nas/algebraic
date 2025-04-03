use primitive_types::U256;

#[derive(Clone, Copy, PartialEq)]
pub struct U256Range{
    current: U256,
    end: U256
}

impl U256Range {
    pub fn new(start: U256, end: U256) -> Self {
        Self { current: start, end }
    }
}

impl Iterator for U256Range {
    type Item = U256;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += U256::one();
            Some(result)
        } else {
            None
        }    
    }
}

pub fn gcd(a: U256, b: U256) -> U256 {
    let mut a = a;
    let mut b = b;
    while b != U256::zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}