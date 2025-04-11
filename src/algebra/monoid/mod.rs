use element::MonoidOps;

use super::{operations::BinaryOperationType, properties::Identity, semigroup::{SemiGroup, SemiGroupOps}, AlgebraicStructure};

pub mod element;
pub mod impls;

/// A trait representing a **monoid** under a binary operation `O`.
///
/// A monoid is an algebraic structure that satisfies:
/// 1. **Closure**: The result of the operation on any two elements is also in the set.
/// 2. **Associativity**: The operation is associative.
/// 3. **Identity**: There exists an identity element for the operation.
///
/// This trait combines the following requirements:
/// - Implements `AlgebraicStructure<O>`
/// - Provides an identity element via `Identity<O, Self>`
/// - The element type supports monoid operations via `MonoidOps`
///
/// The trait also requires `Debug` to facilitate logging and debugging.
pub trait Monoid<O: BinaryOperationType>:
    SemiGroup<O>
    + Identity<O, Self>
where
    <Self as AlgebraicStructure<O>>::Element: SemiGroupOps<O, Self>,
    Self::Element: MonoidOps<O, Self>
{}
