use primitive_types::U256;

use crate::algebra::{operations::{BinaryOperation, BinaryOperationType}, properties::{Associative, Invertible}};

use super::Group;

pub trait ElementFinite<O, G: Group<O>> 
where 
    O: BinaryOperationType<G::Element>
{
    fn element_order(g: &G, element: &G::Element) -> U256;
}

pub trait GroupOps<O, G: Group<O>>:
    Sized
    + PartialEq
    + BinaryOperation<Self>
    + Associative<Self>
    + Invertible<Self> 
where 
    O: BinaryOperationType<G::Element>
{}

