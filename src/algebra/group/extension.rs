use primitive_types::U256;

use crate::algebra::{operations::BinaryOperationType, AlgebraicStructure};

use super::{element::GroupOps, Group};

pub trait Power<O: BinaryOperationType, G: Group<O>> 
where 
    <G as AlgebraicStructure<O>>::Element: GroupOps<O, G>
{
    fn pow(element: &G::Element, exp: U256) -> G::Element;
}

pub trait ScalarMultiplication<S> {
    fn scale(&self, scalar: &S) -> Self;
}