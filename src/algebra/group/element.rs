use primitive_types::U256;

use crate::algebra::{
    operations::{BinaryOperation, BinaryOperationType},
    properties::{Associative, Invertible},
};

use super::Group;

/// A trait for computing the **order of a group element** in a group `G`
/// under the binary operation `O`.
///
/// The **element order** is the smallest positive integer `n` such that:
/// `element^n = identity`, where the exponentiation is defined via repeated group operation.
///
/// This trait is typically used in finite groups.
pub trait ElementFinite<O: BinaryOperationType, G: Group<O>>
where
    G::Element: GroupOps<O, G>,
{
    /// Returns the order of the specified `element` in group `g`.
    fn element_order(g: &G, element: &G::Element) -> U256;
}

/// A trait defining the minimal set of operations an **element of a group**
/// must support under binary operation `O`.
///
/// This trait is required for elements of algebraic structures that are groups
/// and includes:
/// - Binary operation (`BinaryOperation`)
/// - Associativity (`Associative`)
/// - Invertibility (`Invertible`)
///
/// It is designed to support recursive reasoning over group elements.
pub trait GroupOps<O: BinaryOperationType, G: Group<O>>:
    Sized
    + PartialEq
    + BinaryOperation<O, G>
    + Associative<O, G>
    + Invertible<O, G>
where
    G::Element: GroupOps<O, G>,
{}
