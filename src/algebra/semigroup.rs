
use crate::algebra::{
    operations::{BinaryOperation, BinaryOperationType}, 
    properties::Associative, 
    AlgebraicStructure
};

pub trait SemiGroup<O: BinaryOperationType>:
    std::fmt::Debug
    + Sized
    + AlgebraicStructure<O>
where
    Self::Element: SemiGroupOps<O, Self>
{}

pub trait SemiGroupOps<O: BinaryOperationType, S: SemiGroup<O>>:
    Sized
    + PartialEq
    + AlgebraicStructure<O>
    + BinaryOperation<O, Self>
    + Associative<O, Self>
where
    S::Element: SemiGroupOps<O, S>
{}
