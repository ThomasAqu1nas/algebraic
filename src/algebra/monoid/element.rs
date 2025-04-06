use crate::algebra::{
    monoid::Monoid, 
    operations::{
        BinaryOperation, 
        BinaryOperationType
    }, properties::Associative, 
    AlgebraicStructure
};


pub trait MonoidOps<O: BinaryOperationType, M: Monoid<O>>:
    Sized
    + PartialEq
    + AlgebraicStructure
    + BinaryOperation<Self>
    + Associative<Self>
where M::Element: MonoidOps<O, M>
{}


