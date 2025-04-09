use primitive_types::U256;

use super::{
    operations::{Additive, BinaryOperation, BinaryOperationType, Multiplicative},
    AlgebraicStructure,
};

/// A trait that defines the **identity element** for a given binary operation `O`
/// on an algebraic structure `S`.
pub trait Identity<O: BinaryOperationType, S: AlgebraicStructure<O>> {
    /// Returns the identity element of the operation `O`.
    fn identity() -> S::Element;
}

/// A trait for algebraic structures that support **inversion** under a binary operation `O`.
/// The operation must be defined via the `BinaryOperation` trait.
pub trait Invertible<O: BinaryOperationType, S: AlgebraicStructure<O>>: BinaryOperation<O, S> {
    /// Returns the inverse of the given element if it exists.
    /// Returns `None` if the inverse does not exist.
    fn inverse(a: &S::Element) -> Option<S::Element>;
}

/// A trait indicating that an algebraic structure is **finite**.
pub trait Finite {
    /// Returns the number of elements in the structure (its order).
    fn order(&self) -> U256;
}

/// A trait indicating that the binary operation `O` on structure `S` is **commutative**,
/// i.e., `a * b == b * a` for all `a`, `b`.
pub trait Commutative<O: BinaryOperationType, S: AlgebraicStructure<O>>: BinaryOperation<O, S> {
    /// Checks whether the operation is commutative for the given elements.
    fn is_commutative(a: &S::Element, b: &S::Element) -> bool {
        Self::op(a, b) == Self::op(b, a)
    }
}

/// A trait indicating that the binary operation `O` on structure `S` is **associative**,
/// i.e., `(a * b) * c == a * (b * c)` for all `a`, `b`, `c`.
pub trait Associative<O: BinaryOperationType, S: AlgebraicStructure<O>>: BinaryOperation<O, S> {
    /// Checks whether the operation is associative for the given elements.
    fn is_associative(a: &S::Element, b: &S::Element, c: &S::Element) -> bool {
        Self::op(&Self::op(a, b), c) == Self::op(a, &Self::op(b, c))
    }
}

/// A trait for verifying **distributivity** of two operations (typically addition and multiplication)
/// on an algebraic structure `S` with element type `T`.
///
/// This trait requires that the structure supports both additive and multiplicative operations,
/// and that the element type implements both corresponding binary operations.
pub trait Distributive<S, T>
where
    S: AlgebraicStructure<Additive, Element = T> + AlgebraicStructure<Multiplicative, Element = T>,
    T: PartialEq,
    T: BinaryOperation<Additive, S>,
    T: BinaryOperation<Multiplicative, S>,
{
    /// Checks **left distributivity**:
    /// `a * (b + c) == (a * b) + (a * c)`
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

    /// Checks **right distributivity**:
    /// `(b + c) * a == (b * a) + (c * a)`
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

    /// Checks both left and right distributivity.
    fn is_distributive(a: &T, b: &T, c: &T) -> bool {
        Self::is_left_distributive(a, b, c) && Self::is_right_distributive(a, b, c)
    }
}
