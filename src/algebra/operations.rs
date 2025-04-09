use super::AlgebraicStructure;

/// A trait that defines a **binary operation** over an algebraic structure `S`
/// for a given operation type `O`.
///
/// This trait provides a single method `op`, which combines two elements
/// of the structure according to the semantics of `O` (e.g., addition or multiplication).
pub trait BinaryOperation<O: BinaryOperationType, S: AlgebraicStructure<O>> {
    /// Applies the binary operation to the given operands `a` and `b`,
    /// returning the result.
    fn op(a: &S::Element, b: &S::Element) -> S::Element;
}

/// Marker type representing the **additive operation** (e.g., `+`).
pub struct Additive;

/// Marker type representing the **multiplicative operation** (e.g., `*`).
pub struct Multiplicative;

/// A marker trait used to denote types that represent **binary operation kinds**.
///
/// This trait should be implemented for types that are used as operation
/// selectors (such as `Additive`, `Multiplicative`, or custom operations).
pub trait BinaryOperationType {}

impl BinaryOperationType for Additive {}
impl BinaryOperationType for Multiplicative {}
