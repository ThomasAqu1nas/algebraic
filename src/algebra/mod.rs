use operations::{BinaryOperation, BinaryOperationType};
use primitive_types::U256;

pub mod group;
pub mod elliptic_curve;
pub mod field;
pub mod properties;
pub mod operations;
pub mod monoid;
pub mod semigroup;
pub mod ring;

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
