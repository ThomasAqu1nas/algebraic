use operations::{BinaryOperation, BinaryOperationType};
use primitive_types::U256;

pub mod group;
pub mod elliptic_curve;
pub mod jacobian;
pub mod field;
pub mod properties;
pub mod operations;
pub mod monoid;

/// A trait representing a generic **algebraic structure**
/// parameterized by a binary operation `O`.
///
/// This trait defines an associated `Element` type,
/// which must implement the corresponding `BinaryOperation`
/// and be comparable via equality (`PartialEq`).
pub trait AlgebraicStructure<O: BinaryOperationType>: Sized {
    /// The type of elements that belong to the algebraic structure.
    type Element: BinaryOperation<O, Self> + PartialEq;
}

/// A trait for computing the **order of a specific element**
/// in a finite algebraic structure.
///
/// The *element order* is the smallest positive integer `n`
/// such that `a^n = identity`, where `a` is the element.
pub trait ElementFinite<O: BinaryOperationType, S: AlgebraicStructure<O>> {
    /// Returns the order of the given element in the structure `g`.
    fn element_order(g: &S, element: &S::Element) -> U256;
}
