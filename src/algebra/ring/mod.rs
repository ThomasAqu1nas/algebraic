use super::{group::{element::GroupOps, AbelianGroup}, operations::{Additive, BinaryOperation, Multiplicative}, properties::{Commutative, Distributive}, semigroup::{SemiGroup, SemiGroupOps}, AlgebraicStructure};

pub trait Ring<T>: 
    AbelianGroup<Additive>
    + SemiGroup<Multiplicative>
    + AlgebraicStructure<Additive, Element = T>
    + AlgebraicStructure<Multiplicative, Element = T>
where
    T: PartialEq
        + BinaryOperation<Additive, Self>
        + BinaryOperation<Multiplicative, Self>
        + GroupOps<Additive, Self>
        + SemiGroupOps<Multiplicative, Self>
        + Commutative<Additive, Self>
        + Commutative<Multiplicative, Self>
        + Distributive<Self, T>,
    {}