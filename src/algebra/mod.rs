
use operations::{Additive, BinaryOperation, BinaryOperationType, Multiplicative};
use primitive_types::U256;

pub mod group;
pub mod elliptic_curve;
pub mod jacobian;
pub mod field;
pub mod properties;
pub mod operations;
pub mod monoid;


pub trait AlgebraicStructure<O: BinaryOperationType>: Sized {
    type Element: BinaryOperation<O, Self> + PartialEq;
}

pub trait ElementFinite<O: BinaryOperationType, S: AlgebraicStructure<O>> 
{
    fn element_order(g: &S, element: &S::Element) -> U256;
}