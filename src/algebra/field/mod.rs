use super::{group::{element::GroupOps, AbelianGroup, Group}, operations::{Additive, BinaryOperation, Multiplicative}, properties::{Commutative, Distributive, Finite}, AlgebraicStructure};

// type AdditiveElement<F> = <F as AlgebraicStructure>::Element;
// type MultiplicativeElement<F> = <F as AlgebraicStructure>::Element;


pub mod element;
pub mod fp_impl;
pub trait Field: 
    AbelianGroup<Additive>
    + AbelianGroup<Multiplicative>
where 
    Self::Element: GroupOps<Additive, Self>,
    Self::Element: GroupOps<Multiplicative, Self>,
    Self::Element: Distributive<
        Self, 
        <Self as AbelianGroup<Multiplicative>>::Element,
        <Self as AbelianGroup<Additive>>::Element
    >,
    <Self as AlgebraicStructure>::Element: Commutative<Self>
{}

// pub trait Field: 
//     AbelianGroup<Additive>
//     + AbelianGroup<Multiplicative>
// where 
//     Self::Element: GroupOps<Additive, Self>,
//     Self::Element: GroupOps<Multiplicative, Self>,
//     Self::Element: Distributive<
//         Self, 
//         <Self as AbelianGroup<Multiplicative>>::Element,
//         <Self as AbelianGroup<Additive>>::Element
//     >,
//     <Self as AlgebraicStructure>::Element: Commutative<Self>
// {}

// pub trait FiniteField: 
//     Field 
//     + Finite 
// where 
//     Self::Element: GroupOps<Additive, Self> + Finite,
//     Self::Element: GroupOps<Multiplicative, Self> + Finite
// {}
