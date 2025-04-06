use super::AlgebraicStructure;

pub trait BinaryOperation<S: AlgebraicStructure> {
    type BinaryOperationType: BinaryOperationType;
    fn op(a: &S::Element, b: &S::Element) -> S::Element;
}

pub struct Additive;
pub struct Multiplicative;

pub trait BinaryOperationType {}

impl BinaryOperationType for Additive {}
impl BinaryOperationType for Multiplicative {}

