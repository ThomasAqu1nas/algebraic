use primitive_types::U256;

use super::{operations::{Additive, BinaryOperation, BinaryOperationType, Multiplicative}, AlgebraicStructure};

pub trait Identity<O: BinaryOperationType, S: AlgebraicStructure<O>> {
    fn identity() -> S::Element;
}

pub trait Invertible<O: BinaryOperationType, S: AlgebraicStructure<O>>: BinaryOperation<O, S> {
    fn inverse(a: &S::Element) -> Option<S::Element>;
}

pub trait Finite {
    fn order(&self) -> U256;
}


pub trait Commutative<O: BinaryOperationType, S: AlgebraicStructure<O>>: BinaryOperation<O, S>
{
    fn is_commutative(a: &S::Element, b: &S::Element) -> bool {
        Self::op(a, b) == Self::op(b, a)
    }
}

pub trait Associative<O: BinaryOperationType, S: AlgebraicStructure<O>>: BinaryOperation<O, S> 
{
    fn is_associative(a: &S::Element, b: &S::Element, c: &S::Element) -> bool {
        Self::op(&Self::op(a, b), c) == Self::op(a, &Self::op(b, c))
    }
}

/// Трейт для проверки дистрибутивности для алгебраических структур, где операции сложения
/// (Additive) и умножения (Multiplicative) действуют на одном и том же типе элементов T.
/// Для этого вводится параметр T, и требуется, чтобы S реализовывал оба трейта
/// AlgebraicStructure с элементом T.
pub trait Distributive<S, T>
where
    S: AlgebraicStructure<Additive, Element = T> + AlgebraicStructure<Multiplicative, Element = T>,
    T: PartialEq,
    T: BinaryOperation<Additive, S>,
    T: BinaryOperation<Multiplicative, S>,
{
    // Проверяет условие: a * (b + c) == (a * b) + (a * c)
    fn is_left_distributive(a: &T, b: &T, c: &T) -> bool {
        let left = <T as BinaryOperation<Multiplicative, S>>::op(
            a,
            &<T as BinaryOperation<Additive, S>>::op(b, c),
        );
        let right = <T as BinaryOperation<Additive, S>>::op(
            &<T as BinaryOperation<Multiplicative, S>>::op(a, b),
            &<T as BinaryOperation<Multiplicative, S>>::op(a, c),
        );
        left == right
    }

    // Проверяет условие: (b + c) * a == (b * a) + (c * a)
    fn is_right_distributive(a: &T, b: &T, c: &T) -> bool {
        let left = <T as BinaryOperation<Multiplicative, S>>::op(
            &<T as BinaryOperation<Additive, S>>::op(b, c),
            a,
        );
        let right = <T as BinaryOperation<Additive, S>>::op(
            &<T as BinaryOperation<Multiplicative, S>>::op(b, a),
            &<T as BinaryOperation<Multiplicative, S>>::op(c, a),
        );
        left == right
    }

    fn is_distributive(a: &T, b: &T, c: &T) -> bool {
        Self::is_left_distributive(a, b, c) && Self::is_right_distributive(a, b, c)
    }
}