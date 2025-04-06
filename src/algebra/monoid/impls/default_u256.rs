use primitive_types::U256;

use crate::algebra::{
    monoid::{
        element::MonoidOps, 
        Monoid
    }, 
    operations::{
        Additive, 
        BinaryOperation
    }, 
    properties::{Associative, Identity}, 
    AlgebraicStructure
};


impl AlgebraicStructure for U256 {
    type Element = U256;
}

impl Monoid<Additive> for U256 {}
impl MonoidOps<Additive, U256> for U256 {}

impl Associative<U256> for U256 {}

impl BinaryOperation<U256> for U256 {
    type BinaryOperationType = Additive;

    fn op(
        a: &<U256 as AlgebraicStructure>::Element, 
        b: &<U256 as AlgebraicStructure>::Element
    ) -> <U256 as AlgebraicStructure>::Element {
        a + b
    }
}

impl Identity<U256> for U256 {
    fn identity() -> U256 {
        U256::zero()
    }
}