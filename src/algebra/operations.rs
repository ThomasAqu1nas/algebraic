use std::ops::{Add, Mul};

pub trait BinaryOperation<E> {
    type BinaryOperationType: BinaryOperationType<E>;
    fn op(a: &E, b: &E) -> E {
        Self::BinaryOperationType::op(a, b)
    }
}

pub struct Additive;
pub struct Multiplicative;

pub trait BinaryOperationType<E> {
    fn op(a: &E, b: &E) -> E;
}

impl<E: Add<Output = E>> BinaryOperationType<E> for Additive 
where 
    for<'a> &'a E: Add<&'a E, Output = E>
{
    fn op(a: &E, b: &E) -> E {
        a + b
    }
}

impl<E: Mul> BinaryOperationType<E> for Multiplicative 
where 
    for<'a> &'a E: Mul<&'a E, Output = E>
{
    fn op(a: &E, b: &E) -> E {
        a * b
    }
}