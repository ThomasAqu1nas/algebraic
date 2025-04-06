use primitive_types::U256;

use super::{operations::{BinaryOperation, BinaryOperationType}, AlgebraicStructure};

pub trait Identity<S: AlgebraicStructure> {
    fn identity() -> S::Element;
}

pub trait Invertible<S: AlgebraicStructure>: BinaryOperation<S> {
    fn inverse(a: &S::Element) -> S::Element;
}

pub trait Finite {
    fn order(&self) -> U256;
}


pub trait Commutative<S: AlgebraicStructure>: BinaryOperation<S>
{
    fn is_commutative(a: &S::Element, b: &S::Element) -> bool {
        Self::op(a, b) == Self::op(b, a)
    }
}

pub trait Associative<S: AlgebraicStructure>: BinaryOperation<S> 
{
    fn is_associative(a: &S::Element, b: &S::Element, c: &S::Element) -> bool {
        Self::op(&Self::op(a, b), c) == Self::op(a, &Self::op(b, c))
    }
}

pub trait Distributive<S: AlgebraicStructure, OuterOp, InnerOp>: 
    BinaryOperation<S, BinaryOperationType = OuterOp>
where
    OuterOp: BinaryOperationType,
    InnerOp: BinaryOperationType,
{
    fn is_left_distributive(a: &S::Element, b: &S::Element, c: &S::Element) -> bool {
        let left = S::Element::op(a, &InnerOp::op(b, c));
        let right = InnerOp::op(&OuterOp::op(a, b), &OuterOp::op(a, c));
        left == right
    }

    fn is_right_distributive(a: &S::Element, b: &S::Element, c: &S::Element) -> bool {
        let left = OuterOp::op(&InnerOp::op(b, c), a);
        let right = InnerOp::op(&OuterOp::op(b, a), &OuterOp::op(c, a));
        left == right
    }

    fn is_distributive(a: &S::Element, b: &S::Element, c: &S::Element) -> bool {
        Self::is_left_distributive(a, b, c) && Self::is_right_distributive(a, b, c)
    }
}