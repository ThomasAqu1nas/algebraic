use primitive_types::U256;

/// A struct representing a range over `U256` values.
/// Implements the `Iterator` trait, yielding values from `current` up to (but not including) `end`.
#[derive(Clone, Copy, PartialEq)]
pub struct U256Range {
    current: U256,
    end: U256,
}

impl U256Range {
    /// Creates a new `U256Range` from `start` to `end`.
    pub fn new(start: U256, end: U256) -> Self {
        Self { current: start, end }
    }
}

impl Iterator for U256Range {
    type Item = U256;

    /// Returns the next value in the range, or `None` if the end has been reached.
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

/// Computes the greatest common divisor (GCD) of two `U256` values using the Euclidean algorithm.
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

/// Extended Euclidean Algorithm.
/// Returns a triple `(g, x, y)`, where `g = gcd(a, b)` and `x`, `y` satisfy the equation:
/// `a * x + b * y = g`.
pub fn extended_gcd(a: U256, b: U256) -> (U256, i128, i128) {
    let mut r0 = a;
    let mut r1 = b;
    let mut s0: i128 = 1;
    let mut s1: i128 = 0;
    let mut t0: i128 = 0;
    let mut t1: i128 = 1;

    while r1 != U256::zero() {
        let q = r0 / r1;
        let r = r0 % r1;
        r0 = r1;
        r1 = r;

        // Convert q to i128 (this is fine as long as q fits)
        let q_i128 = q.as_u128() as i128;

        let s = s0 - q_i128 * s1;
        s0 = s1;
        s1 = s;

        let t = t0 - q_i128 * t1;
        t0 = t1;
        t1 = t;
    }
    (r0, s0, t0)
}

/// Computes the modular inverse of `a` modulo `m`.
///
/// If the inverse exists (i.e., `a` and `m` are coprime),
/// returns `Some(inverse)`, where `inverse` is in the range `[0, m - 1]`.
/// Otherwise, returns `None`.
pub fn mod_inverse(a: U256, m: U256) -> Option<U256> {
    let (g, x, _) = extended_gcd(a, m);
    // If gcd(a, m) != 1, then no modular inverse exists
    if g != U256::from(1u8) {
        return None;
    }
    // Ensure the result is positive in modulo m space.
    // Using (x mod m + m) mod m to handle negative x values.
    // Note: x is i128 and m is U256. This conversion assumes m fits into i128.
    let m_i128 = m.as_u128() as i128; // Valid if m fits in i128
    let mut x_i128 = x % m_i128;
    if x_i128 < 0 {
        x_i128 += m_i128;
    }
    Some(U256::from(x_i128 as u128))
}
