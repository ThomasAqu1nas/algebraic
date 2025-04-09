use super::AlgebraicStructure;

pub trait BinaryOperation<O: BinaryOperationType, S: AlgebraicStructure<O>> {
    fn op(a: &S::Element, b: &S::Element) -> S::Element;
}

pub struct Additive;
pub struct Multiplicative;
pub struct Complex;

pub trait BinaryOperationType {}

impl BinaryOperationType for Additive {}
impl BinaryOperationType for Multiplicative {}

