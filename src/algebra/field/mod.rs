use super::{group::{element::GroupOps, AbelianGroup}, operations::{Additive, BinaryOperation, Multiplicative}, properties::{Commutative, Distributive, Finite}, AlgebraicStructure};

// type AdditiveElement<F> = <F as AlgebraicStructure>::Element;
// type MultiplicativeElement<F> = <F as AlgebraicStructure>::Element;


pub mod element;
pub mod fp_impls;

pub trait Field<T>: 
    AbelianGroup<Additive>
    + AbelianGroup<Multiplicative>
    + AlgebraicStructure<Additive, Element = T>
    + AlgebraicStructure<Multiplicative, Element = T>
where
    T: PartialEq
       + BinaryOperation<Additive, Self>
       + BinaryOperation<Multiplicative, Self>
       + GroupOps<Additive, Self>
       + GroupOps<Multiplicative, Self>
       + Commutative<Additive, Self>
       + Commutative<Multiplicative, Self>
       + Distributive<Self, T>,
{}


pub trait FiniteField<T>: 
    Field<T> 
    + Finite 
where 
    T: PartialEq
        + BinaryOperation<Additive, Self>
        + BinaryOperation<Multiplicative, Self>
        + GroupOps<Additive, Self>
        + GroupOps<Multiplicative, Self>
        + Commutative<Additive, Self>
        + Commutative<Multiplicative, Self>
        + Distributive<Self, T>
        + Finite,
{}
