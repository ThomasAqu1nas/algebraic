use primitive_types::U256;

use crate::algebra::{operations::BinaryOperationType, AlgebraicStructure};

use super::{element::GroupOps, Group};

/// A trait that defines **exponentiation (repeated group operation)**
/// for an element in a group `G` under binary operation `O`.
///
/// Typically used for computing `element^exp` in multiplicative groups,
/// or `exp * element` in additive groups.
pub trait Power<O: BinaryOperationType, G: Group<O>>
where
    <G as AlgebraicStructure<O>>::Element: GroupOps<O, G>,
{
    /// Raises the given `element` to the power of `exp`
    /// using the group operation defined by `O`.
    fn pow(element: &G::Element, exp: U256) -> G::Element;
}

/// A trait representing **scalar multiplication**
/// where a value (typically a group element) is scaled by a scalar value.
///
/// This is a generic trait and is commonly used in elliptic curve arithmetic
/// and other algebraic settings where an element can be multiplied by a scalar.
pub trait ScalarMultiplication<S> {
    /// Returns the result of scaling `self` by the given `scalar`.
    fn scale(&self, scalar: &S) -> Self;
}
