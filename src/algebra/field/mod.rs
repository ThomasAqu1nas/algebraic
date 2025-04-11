use super::{
    group::{element::GroupOps, AbelianGroup},
    operations::{Additive, BinaryOperation, Multiplicative},
    properties::{Commutative, Distributive, Finite},
    AlgebraicStructure,
};

pub mod operation_helpers;
pub mod fp_impls;

/// A trait representing a mathematical **field** over elements of type `T`.
///
/// A field is an algebraic structure that satisfies:
/// - An abelian group under addition (`Additive`)
/// - An abelian group under multiplication (`Multiplicative`), excluding zero
/// - Distributivity of multiplication over addition
///
/// This trait assumes that the type `Self` implements both additive and multiplicative group
/// structures over a shared element type `T`, and that `T` supports all required operations.
pub trait Field<T>:
    AbelianGroup<Additive>
    + AbelianGroup<Multiplicative>
    + AlgebraicStructure<Additive, Element = T>
    + AlgebraicStructure<Multiplicative, Element = T>
where
    T: PartialEq
        + BinaryOperation<Additive, Self>
        + BinaryOperation<Multiplicative, Self>
        + GroupOps<Additive, Self>
        + GroupOps<Multiplicative, Self>
        + Commutative<Additive, Self>
        + Commutative<Multiplicative, Self>
        + Distributive<Self, T>,
{}

/// A trait representing a **finite field** (also called a Galois field).
///
/// This is a field with a finite number of elements. The structure `Self`
/// must implement both field behavior and provide a way to determine its order
/// (i.e. total number of distinct elements).
pub trait FiniteField<T>:
    Field<T>
    + Finite
where
    T: PartialEq
        + BinaryOperation<Additive, Self>
        + BinaryOperation<Multiplicative, Self>
        + GroupOps<Additive, Self>
        + GroupOps<Multiplicative, Self>
        + Commutative<Additive, Self>
        + Commutative<Multiplicative, Self>
        + Distributive<Self, T>
        + Finite,
{}
