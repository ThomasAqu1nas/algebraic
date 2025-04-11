use primitive_types::U256;

use crate::algebra::{
    monoid::{
        element::MonoidOps, 
        Monoid
    }, operations::{
        Additive, 
        BinaryOperation
    }, properties::{Associative, Identity}, semigroup::{SemiGroup, SemiGroupOps}, AlgebraicStructure
};


impl AlgebraicStructure<Additive> for U256 {
    type Element = U256;
}

impl Monoid<Additive> for U256 {}
impl SemiGroup<Additive> for U256 {}
impl SemiGroupOps<Additive, U256> for U256 {}
impl MonoidOps<Additive, U256> for U256 {}

impl Associative<Additive, U256> for U256 {}

impl BinaryOperation<Additive, U256> for U256 {
    fn op(
        a: &<U256 as AlgebraicStructure<Additive>>::Element, 
        b: &<U256 as AlgebraicStructure<Additive>>::Element
    ) -> <U256 as AlgebraicStructure<Additive>>::Element {
        a + b
    }
}

impl Identity<Additive, U256> for U256 {
    fn identity() -> U256 {
        U256::zero()
    }
}