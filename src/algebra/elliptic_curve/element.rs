use crate::algebra::{field::FiniteField, group::{element::{ElementFinite, GroupOps}, Group}, operations::{Additive, Multiplicative}, properties::{Commutative, Distributive, Finite}, AlgebraicStructure};

use super::EllipticCurve;


pub trait EllipticCurvePoint<
    T: GroupOps<Additive, F> 
        + GroupOps<Multiplicative, F>
        + Commutative<Additive, F>
        + Commutative<Multiplicative, F>
        + Distributive<F, T>
        + Finite, 
    F: FiniteField<T>,
    E: EllipticCurve<T, F>
>: 
    GroupOps<Additive, E> 
    + Commutative<Additive, Self> 
    + AlgebraicStructure<Additive> 
    + ElementFinite<Additive, E>
    + Clone
where 
    <E as AlgebraicStructure<Additive>>::Element:
        EllipticCurvePoint<T, F, E>
        + Commutative<Additive, E>
{
    fn add(&self, rhs: &Self) -> Self;
    fn double(&self) -> Self;
    fn mul_scalar(&self) -> Self;
    fn is_on_curve(&self, curve: &E) -> bool;
    fn is_identity(&self) -> bool;
    fn neg(&self) -> Self;
}

