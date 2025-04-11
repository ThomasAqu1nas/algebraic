use crate::algebra::{group::element::GroupOps, operations::{Additive, BinaryOperation, Multiplicative}, properties::{Commutative, Distributive, Invertible}};

use super::Field;

// semantic helpers
#[inline]
pub fn mul<F: Field<T>, T>(a: &T, b: &T) -> T
where
    T: Clone
    + GroupOps<Additive, F>
    + GroupOps<Multiplicative, F>
    + BinaryOperation<Additive, F>
    + BinaryOperation<Multiplicative, F>
    + Commutative<Additive, F>
    + Commutative<Multiplicative, F>
    + Distributive<F, T>
{
    <T as BinaryOperation<Multiplicative, F>>::op(a, b)
}

#[inline]
fn add<F: Field<T>, T>(a: &T, b: &T) -> T
where
    T: Clone
        + GroupOps<Additive, F>
        + GroupOps<Multiplicative, F>
        + BinaryOperation<Additive, F>
        + BinaryOperation<Multiplicative, F>
        + Commutative<Additive, F>
        + Commutative<Multiplicative, F>
        + Distributive<F, T>
{
    <T as BinaryOperation<Additive, F>>::op(a, b)
}

#[inline]
pub fn sub<F: Field<T>, T>(a: &T, b: &T) -> T
where
    T: Clone
        + GroupOps<Additive, F>
        + GroupOps<Multiplicative, F>
        + BinaryOperation<Additive, F>
        + BinaryOperation<Multiplicative, F>
        + Commutative<Additive, F>
        + Commutative<Multiplicative, F>
        + Distributive<F, T>
{
    <T as BinaryOperation<Additive, F>>::op(
        a, 
        &<T as Invertible<Additive, F>>::inverse(b)
            .unwrap()
    )
}


#[inline]
pub fn double<F: Field<T>, T>(a: &T) -> T
where
    T: Clone
        + GroupOps<Additive, F>
        + GroupOps<Multiplicative, F>
        + BinaryOperation<Additive, F>
        + BinaryOperation<Multiplicative, F>
        + Commutative<Additive, F>
        + Commutative<Multiplicative, F>
        + Distributive<F, T>
{
    add::<F, T>(a, a)
}