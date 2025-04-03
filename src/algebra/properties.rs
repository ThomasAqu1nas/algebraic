use primitive_types::U256;

use super::operations::{BinaryOperation, BinaryOperationType};

pub trait Identity<E> {
    fn identity() -> E;
}

pub trait Invertible<E>: BinaryOperation<E> {
    fn inverse(a: &E) -> E;
}

pub trait Finite {
    fn order(&self) -> U256;
}


pub trait Commutative<T>: BinaryOperation<T>
where
    T: PartialEq,
{
    fn is_commutative(a: &T, b: &T) -> bool {
        Self::op(a, b) == Self::op(b, a)
    }
}

pub trait Associative<T>: BinaryOperation<T> 
where 
    T: PartialEq
{
    fn is_associative(a: &T, b: &T, c: &T) -> bool {
        Self::op(&Self::op(a, b), c) == Self::op(a, &Self::op(b, c))
    }
}

pub trait Distributive<T, OuterOp, InnerOp>: 
    BinaryOperation<T, BinaryOperationType = OuterOp>
where
    OuterOp: BinaryOperationType<T>,
    InnerOp: BinaryOperationType<T>,
    T: PartialEq,
{
    fn is_left_distributive(a: &T, b: &T, c: &T) -> bool {
        let left = OuterOp::op(a, &InnerOp::op(b, c));
        let right = InnerOp::op(&OuterOp::op(a, b), &OuterOp::op(a, c));
        left == right
    }

    fn is_right_distributive(a: &T, b: &T, c: &T) -> bool {
        let left = OuterOp::op(&InnerOp::op(b, c), a);
        let right = InnerOp::op(&OuterOp::op(b, a), &OuterOp::op(c, a));
        left == right
    }

    fn is_distributive(a: &T, b: &T, c: &T) -> bool {
        Self::is_left_distributive(a, b, c) && Self::is_right_distributive(a, b, c)
    }
}