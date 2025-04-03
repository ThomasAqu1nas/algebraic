use primitive_types::U256;

use crate::algebra::operations::BinaryOperationType;

use super::Group;

pub trait Power<O, G: Group<O>> 
where 
    O: BinaryOperationType<G::Element>
{
    fn pow(element: &G::Element, exp: U256) -> G::Element;
}

pub trait ScalarMultiplication<S> {
    fn scale(&self, scalar: &S) -> Self;
}
