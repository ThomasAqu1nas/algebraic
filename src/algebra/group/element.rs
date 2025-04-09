use primitive_types::U256;

use crate::algebra::{operations::{BinaryOperation, BinaryOperationType}, properties::{Associative, Invertible}};

use super::Group;

pub trait ElementFinite<O: BinaryOperationType, G: Group<O>> 
where 
    G::Element: GroupOps<O, G>
{
    fn element_order(g: &G, element: &G::Element) -> U256;
}

pub trait GroupOps<O: BinaryOperationType, G: Group<O>>:
    Sized
    + PartialEq
    + BinaryOperation<O, G>
    + Associative<O, G>
    + Invertible<O, G>
where 
    G::Element: GroupOps<O, G>
{}


