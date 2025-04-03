use std::ops::{Add, Mul};

use super::{group::Group, operations::{Additive, Multiplicative}, properties::{Commutative, Distributive, Finite}};

pub mod element;

pub trait Field: 
    Group<Additive>
    + Group<Multiplicative>
where 
    for<'a> &'a <Self as Group<Multiplicative>>::Element: 
    Mul<Output = <
        Self as Group<Multiplicative>>::Element
    > 
    + Commutative<<Self as Group<Multiplicative>>::Element>
    + Add<Output = <
        Self as Group<Multiplicative>>::Element
    >
    + Distributive<<Self as Group<Multiplicative>>::Element, Multiplicative, Additive>,
    //
    for<'a> &'a <Self as Group<Additive>>::Element: Add<Output = <
        Self as Group<Additive>>::Element
    > 
    + Commutative<<Self as Group<Additive>>::Element>,
    //
    <Self as Group<Multiplicative>>::Element: Mul<Output = <
        Self as Group<Multiplicative>>::Element
    >
    + Commutative<<Self as Group<Additive>>::Element>
    + Add<Output = <
        Self as Group<Multiplicative>>::Element
    >
    + Distributive<<Self as Group<Multiplicative>>::Element, Multiplicative, Additive>,
    //
    <Self as Group<Additive>>::Element: Add<Output = <
        Self as Group<Additive>>::Element
    >
    + Commutative<<Self as Group<Additive>>::Element>,
{}


pub trait FiniteField: 
    Field 
    + Finite 
where 
    for<'a> &'a <Self as Group<Multiplicative>>::Element: 
    Mul<Output = <
        Self as Group<Multiplicative>>::Element
    > 
    + Commutative<<Self as Group<Multiplicative>>::Element>
    + Add<Output = <
        Self as Group<Multiplicative>>::Element
    >
    + Distributive<<Self as Group<Multiplicative>>::Element, Multiplicative, Additive>
    + Finite,
    //
    for<'a> &'a <Self as Group<Additive>>::Element: Add<Output = <
        Self as Group<Additive>>::Element
    > 
    + Commutative<<Self as Group<Additive>>::Element>
    + Finite,
    //
    <Self as Group<Multiplicative>>::Element: Mul<Output = <
        Self as Group<Multiplicative>>::Element
    >
    + Commutative<<Self as Group<Additive>>::Element>
    + Add<Output = <
        Self as Group<Multiplicative>>::Element
    >
    + Distributive<<Self as Group<Multiplicative>>::Element, Multiplicative, Additive>
    + Finite,
    //
    <Self as Group<Additive>>::Element: Add<Output = <
        Self as Group<Additive>>::Element
    >
    + Commutative<<Self as Group<Additive>>::Element>
    + Finite,
{}