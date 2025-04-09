use element::MonoidOps;

use super::{operations::BinaryOperationType, properties::Identity, AlgebraicStructure};

pub mod element;
pub mod impls;

pub trait Monoid<O: BinaryOperationType>: 
    std::fmt::Debug
    + Sized
    + AlgebraicStructure<O>
    + Identity<O, Self>
where 
    Self::Element: MonoidOps<O, Self>
{}