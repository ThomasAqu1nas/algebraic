#![allow(non_camel_case_types)]
use primitive_types::U256;

use crate::algebra::{group::Group, operations::{Additive, BinaryOperation, Multiplicative}, properties::{Finite, Identity}, AlgebraicStructure};

use super::{Field, FiniteField};

pub struct Fp_7 {
    pub value: U256
}

impl AlgebraicStructure for Fp_7 {
    type Element = U256;
}

impl BinaryOperation<Fp_7> for U256 {
    type BinaryOperationType = Additive;

    fn op(
        a: &<Fp_7 as AlgebraicStructure>::Element, 
        b: &<Fp_7 as AlgebraicStructure>::Element
    ) -> <Fp_7 as AlgebraicStructure>::Element {
        (a + b) % U256::from(7)
    }
}

impl Fp_7 {
    pub fn new(
        value: U256
    ) -> Self {
        Self { value: value % 7 }
    }
}

impl Finite for Fp_7 {
    fn order(&self) -> U256 {
        U256::from(7)
    }
}

impl Field for Fp_7 {

}

impl Group<Multiplicative> for Fp_7 {

}

impl Identity<<Fp_7 as Group<Multiplicative>>> for Fp_7 {
    fn identity() -> <Fp_7 as AlgebraicStructure>::Element {
        
    }
}