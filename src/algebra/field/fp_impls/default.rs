use primitive_types::U256;

use crate::algebra::properties::Finite;

// This implementation might be used to satisfy trait bounds elsewhere.
impl Finite for U256 {
    /// Here, U256’s order is defined as U256::MAX; this is likely a placeholder.
    fn order(&self) -> U256 {
        U256::MAX
    }
}