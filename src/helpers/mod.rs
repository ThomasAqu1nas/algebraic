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

/// Расширенный алгоритм Евклида.
/// Возвращает тройку (g, x, y), где g = gcd(a, b)
/// и x, y удовлетворяют равенству: a * x + b * y = g.
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

        // Приводим q к i128 (это подходит, если q не велик)
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

/// Функция для вычисления обратного элемента a по модулю m.
///
/// Если обратный элемент существует (то есть a и m взаимно просты),
/// возвращает Some(inverse), где inverse принадлежит диапазону [0, m–1].
/// Иначе – None.
pub fn mod_inverse(a: U256, m: U256) -> Option<U256> {
    let (g, x, _) = extended_gcd(a, m);
    // Если gcd(a, m) != 1, то обратного элемента не существует
    if g != U256::from(1u8) {
        return None;
    }
    // Приводим x к положительному результату по модулю m.
    // Здесь для наглядности используем формулу (x mod m + m) mod m.
    // Заметим, что x имеет тип i128, а m – U256.
    // В реальной криптографической библиотеке U256 может быть больше i128,
    // поэтому может потребоваться работа с большими целыми или иная библиотека.
    let m_i128 = m.as_u128() as i128; // Работает корректно, если m вписывается в i128
    let mut x_i128 = x % m_i128;
    if x_i128 < 0 {
        x_i128 += m_i128;
    }
    Some(U256::from(x_i128 as u128))
}
