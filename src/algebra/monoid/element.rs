use crate::algebra::{
    monoid::Monoid, 
    operations::{BinaryOperation, BinaryOperationType}, 
    properties::Associative, 
    AlgebraicStructure
};

/// A trait defining **monoid-level operations** for elements of a monoid structure.
///
/// This trait is intended to be implemented by element types of a monoid `M`
/// under a binary operation `O`. It ensures that the element type:
///
/// - Is associated with a binary operation (`BinaryOperation`)
/// - Implements the `AlgebraicStructure` trait for the operation
/// - Satisfies **associativity** (`Associative`)
/// - Supports comparison (`PartialEq`)
///
/// This trait enables recursive and generic manipulation of monoid elements.
pub trait MonoidOps<O: BinaryOperationType, M: Monoid<O>>:
    Sized
    + PartialEq
    + AlgebraicStructure<O>
    + BinaryOperation<O, Self>
    + Associative<O, Self>
where
    M::Element: MonoidOps<O, M>
{}
