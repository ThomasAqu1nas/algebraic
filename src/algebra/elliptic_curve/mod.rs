use element::EllipticCurvePoint;

use super::{field::FiniteField, group::{element::{ElementFinite, GroupOps}, CyclingGroup}, operations::{Additive, Multiplicative}, properties::{Commutative, Distributive, Finite}, AlgebraicStructure};

pub mod impls;
pub mod element;

pub trait EllipticCurve<
    T: GroupOps<Additive, F> 
        + GroupOps<Multiplicative, F>
        + Commutative<Additive, F>
        + Commutative<Multiplicative, F>
        + Distributive<F, T>
        + Finite, 
    F: FiniteField<T>
>: CyclingGroup<Additive> 
where 
    <Self as AlgebraicStructure<Additive>>::Element: 
        EllipticCurvePoint<T, F, Self> + Commutative<Additive, Self>,
{
    fn a(&self) -> T;
    fn b(&self) -> T;
}



